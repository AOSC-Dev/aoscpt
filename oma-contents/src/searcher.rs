use std::{
    borrow::Cow,
    fs,
    io::{self, BufRead, BufReader, Read, Seek},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self},
};

use flate2::bufread::GzDecoder;
use lzzzz::lz4f::BufReadDecompressor;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::debug;
use zstd::Decoder;

use crate::{parser::single_line, OmaContentsError};

const ZSTD_MAGIC: &[u8] = &[40, 181, 47, 253];
const LZ4_MAGIC: &[u8] = &[0x04, 0x22, 0x4d, 0x18];
const GZIP_MAGIC: &[u8] = &[0x1F, 0x8B];

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Provides,
    Files,
    BinProvides,
    BinFiles,
}

const BIN_PREFIX: &str = "usr/bin";

impl Mode {
    fn paths(&self, dir: &Path) -> Result<Vec<PathBuf>, OmaContentsError> {
        use std::fs;

        #[cfg(feature = "aosc")]
        let contains_name = match self {
            Mode::Provides | Mode::Files => "_Contents-",
            Mode::BinProvides | Mode::BinFiles => "_BinContents-",
        };

        #[cfg(not(feature = "aosc"))]
        let contains_name = "_Contents-";

        let mut paths = vec![];

        for i in fs::read_dir(dir)
            .map_err(|e| OmaContentsError::FailedToOperateDirOrFile(dir.display().to_string(), e))?
            .flatten()
        {
            if i.file_name()
                .into_string()
                .is_ok_and(|x| x.contains(contains_name))
            {
                paths.push(i.path());
            }
        }

        Ok(paths)
    }
}

pub fn ripgrep_search(
    dir: impl AsRef<Path>,
    mode: Mode,
    query: &str,
    cb: impl Fn(usize) + Sync + Send + 'static,
) -> Result<Vec<(String, String)>, OmaContentsError> {
    let query = regex::escape(query);

    let query = if Path::new(&query).is_absolute() {
        Cow::Borrowed(query.strip_prefix('/').unwrap_or(&query))
    } else {
        Cow::Owned(query)
    };

    let (regex, is_list) = match mode {
        Mode::Provides | Mode::BinProvides => {
            (format!(r"^(.*?{query}(?:.*[^\s])?)\s+(\S+)\s*$"), false)
        }
        Mode::Files | Mode::BinFiles => (
            format!(r"^\s*(.*?)\s+((?:\S*[,/])?{query}(?:,\S*|))\s*$"),
            true,
        ),
    };

    let mut res = vec![];

    let mut cmd = Command::new("rg")
        .arg("-N")
        .arg("-I")
        .args(mode.paths(dir.as_ref())?)
        .arg("--search-zip")
        .arg("-e")
        .arg(regex)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(OmaContentsError::ExecuteRgFailed)?;

    let stdout = cmd
        .stdout
        .as_mut()
        .expect("Unexpected error: can not get stdout, maybe you environment is broken?");

    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    let mut has_result = false;

    for (i, c) in stdout_lines.map_while(io::Result::ok).enumerate() {
        if let Some(line) = rg_filter_line(&c, is_list, &query) {
            // TODO: 实现输入时 filter

            #[cfg(not(feature = "aosc"))]
            match mode {
                Mode::BinFiles | Mode::BinProvides => {
                    if !line.1.starts_with(BIN_PREFIX) {
                        continue;
                    }
                }
                _ => {}
            }

            has_result = true;

            cb(i + 1);
            if !res.contains(&line) {
                res.push(line);
            }
        }
    }

    if !has_result {
        return Err(OmaContentsError::NoResult);
    }

    if !cmd
        .wait()
        .map_err(OmaContentsError::FailedToWaitExit)?
        .success()
    {
        return Err(OmaContentsError::RgWithError);
    }

    Ok(res)
}

pub fn pure_search(
    path: impl AsRef<Path>,
    mode: Mode,
    query: &str,
    cb: impl Fn(usize) + Sync + Send + 'static,
) -> Result<Vec<(String, String)>, OmaContentsError> {
    let paths = mode.paths(path.as_ref())?;
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();

    let query = Arc::from(query);

    let worker = thread::spawn(move || {
        paths
            .par_iter()
            .map(
                move |path| -> Result<Vec<(String, String)>, OmaContentsError> {
                    pure_search_contents_from_path(path, &query, count.clone(), mode)
                },
            )
            .collect::<Result<Vec<_>, OmaContentsError>>()
            .map(|x| x.into_iter().flatten().collect::<Vec<_>>())
    });

    let mut tmp = 0;
    loop {
        let count = count_clone.load(Ordering::SeqCst);
        if count > 0 && count != tmp {
            cb(count);
            tmp = count;
        }

        if worker.is_finished() {
            return worker.join().unwrap();
        }
    }
}

fn pure_search_contents_from_path(
    path: &Path,
    query: &str,
    count: Arc<AtomicUsize>,
    mode: Mode,
) -> Result<Vec<(String, String)>, OmaContentsError> {
    let mut f = fs::File::open(path)
        .map_err(|e| OmaContentsError::FailedToOperateDirOrFile(path.display().to_string(), e))?;

    let mut buf = [0; 4];
    f.read_exact(&mut buf).ok();
    f.rewind().map_err(|e| {
        debug!("{e}");
        OmaContentsError::IllegalFile(path.display().to_string())
    })?;

    let ext = path.extension().and_then(|x| x.to_str());

    let contents_reader: &mut dyn Read = match ext {
        Some("zst") => {
            check_file_magic_4bytes(buf, path, ZSTD_MAGIC)?;
            // https://github.com/gyscos/zstd-rs/issues/281
            &mut Decoder::new(BufReader::new(f)).unwrap()
        }
        Some("lz4") => {
            check_file_magic_4bytes(buf, path, LZ4_MAGIC)?;
            &mut BufReadDecompressor::new(BufReader::new(f))?
        }
        Some("gz") => {
            if buf[..2] != *GZIP_MAGIC {
                return Err(OmaContentsError::IllegalFile(path.display().to_string()));
            }
            &mut GzDecoder::new(BufReader::new(f))
        }
        _ => &mut BufReader::new(f),
    };

    let reader = BufReader::new(contents_reader);

    let cb = match mode {
        Mode::Provides => |_pkg: &str, file: &str, query: &str| file.contains(query),
        Mode::Files => |pkg: &str, _file: &str, query: &str| pkg == query,
        Mode::BinProvides => |_pkg: &str, file: &str, query: &str| {
            file.contains(query) && file.starts_with(BIN_PREFIX)
        },
        Mode::BinFiles => {
            |pkg: &str, file: &str, query: &str| pkg == query && file.starts_with(BIN_PREFIX)
        }
    };

    let res = pure_search_foreach_result(cb, reader, count, query);

    Ok(res)
}

#[inline]
fn check_file_magic_4bytes(
    buf: [u8; 4],
    path: &Path,
    magic: &[u8],
) -> Result<(), OmaContentsError> {
    if buf != magic {
        return Err(OmaContentsError::IllegalFile(path.display().to_string()));
    }

    Ok(())
}

fn pure_search_foreach_result(
    cb: impl Fn(&str, &str, &str) -> bool,
    reader: BufReader<&mut dyn Read>,
    count: Arc<AtomicUsize>,
    query: &str,
) -> Vec<(String, String)> {
    let mut res = vec![];

    for i in reader.lines() {
        let i = match i {
            Ok(i) => i,
            Err(_) => continue,
        };

        let (file, pkgs) = match single_line::<()>(&mut i.as_str()) {
            Ok(line) => line,
            Err(_) => continue,
        };

        for (_, pkg) in pkgs {
            if cb(pkg, file, query) {
                count.fetch_add(1, Ordering::SeqCst);
                let line = (pkg.to_string(), prefix(file));
                if !res.contains(&line) {
                    res.push(line);
                }
            }
        }
    }

    res
}

fn rg_filter_line(mut line: &str, is_list: bool, query: &str) -> Option<(String, String)> {
    let (file, pkgs) = single_line::<()>(&mut line).ok()?;

    debug!("file: {file}, pkgs: {pkgs:?}");

    if pkgs.len() != 1 {
        for (_, pkg) in pkgs {
            if pkg == query || !is_list {
                let file = prefix(file);
                return Some((pkg.to_string(), file));
            }
        }
    } else {
        // 比如 /usr/bin/apt admin/apt
        let (_, pkg) = pkgs[0];
        let file = prefix(file);
        return Some((pkg.to_string(), file));
    }

    None
}

#[inline]
fn prefix(s: &str) -> String {
    if s.starts_with('/') {
        s.to_string()
    } else {
        "/".to_owned() + s
    }
}
