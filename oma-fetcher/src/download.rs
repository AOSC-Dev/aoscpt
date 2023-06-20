use std::io::SeekFrom;

use indicatif::ProgressBar;
use oma_console::writer::Writer;
use reqwest::{
    header::{HeaderValue, ACCEPT_RANGES, CONTENT_LENGTH, RANGE},
    Client, StatusCode,
};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use crate::{checksum::Checksum, DownloadEntry, DownloadError, DownloadResult, FetchProgressBar};

/// Download file
/// Return bool is file is started download
pub(crate) async fn http_download(
    client: &Client,
    entry: &DownloadEntry,
    fpb: Option<FetchProgressBar>,
) -> DownloadResult<bool> {
    let file = entry.dir.join(&entry.filename);
    let file_exist = file.exists();
    let mut file_size = file.metadata().ok().map(|x| x.len()).unwrap_or(0);

    tracing::debug!("Exist file size is: {file_size}");
    let mut dest = None;
    let mut validator = None;

    // 如果要下载的文件已经存在，则验证 Checksum 是否正确，若正确则添加总进度条的进度，并返回
    // 如果不存在，则继续往下走
    if file_exist {
        tracing::debug!(
            "File: {} exists, oma will checksum this file.",
            entry.filename
        );
        let hash = entry.hash.clone();
        if let Some(hash) = hash {
            tracing::debug!("Hash exist! It is: {hash}");

            let mut f = tokio::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&file)
                .await?;

            tracing::debug!(
                "oma opened file: {} with create, write and read mode",
                entry.filename
            );

            let mut v = Checksum::from_sha256_str(&hash)?.get_validator();

            tracing::debug!("Validator created.");

            let mut buf = vec![0; 4096];
            let mut readed = 0;

            loop {
                if readed == file_size {
                    break;
                }

                let count = f.read(&mut buf[..]).await?;
                v.update(&buf[..count]);

                if let Some(ref fpb) = fpb {
                    if let Some(ref gpb) = fpb.global_bar {
                        gpb.inc(count as u64);
                    }
                }

                readed += count as u64;
            }

            if v.finish() {
                tracing::debug!(
                    "{} checksum success, no need to download anything.",
                    entry.filename
                );
                return Ok(false);
            }

            tracing::debug!("checksum fail, will download this file: {}", entry.filename);

            if !entry.allow_resume {
                if let Some(ref gpb) = fpb.as_ref().and_then(|x| x.global_bar.clone()) {
                    gpb.set_position(gpb.position() - readed);
                }
            } else {
                dest = Some(f);
                validator = Some(v);
            }
        }
    }

    let fpbc = fpb.clone();
    let progress = if let Some((count, len)) = fpbc.and_then(|x| x.progress) {
        format!("({count}/{len}) ")
    } else {
        "".to_string()
    };

    let progress_clone = progress.clone();

    // 若请求头的速度太慢，会看到 Spinner 直到拿到头的信息
    let fpbc = fpb.clone();
    let pb = if let Some(fpb) = fpbc {
        let (style, inv) = oma_console::pb::oma_spinner(false)?;
        let pb = fpb.mb.add(ProgressBar::new_spinner().with_style(style));
        pb.enable_steady_tick(inv);
        let msg = fpb.msg.unwrap_or("todo".to_string());
        pb.set_message(format!("{progress_clone}{msg}"));

        Some(pb)
    } else {
        None
    };

    let url = entry.url.clone();
    let resp_head = client.head(url).send().await?;

    let head = resp_head.headers();

    // 看看头是否有 ACCEPT_RANGES 这个变量
    // 如果有，而且值不为 none，则可以断点续传
    // 反之，则不能断点续传
    let mut can_resume = match head.get(ACCEPT_RANGES) {
        Some(x) if x == "none" => false,
        Some(_) => true,
        None => false,
    };

    tracing::debug!("Can resume? {can_resume}");

    // 从服务器获取文件的总大小
    let total_size = {
        let total_size = head
            .get(CONTENT_LENGTH)
            .map(|x| x.to_owned())
            .unwrap_or(HeaderValue::from(0));

        let url = entry.url.clone();

        total_size
            .to_str()
            .ok()
            .and_then(|x| x.parse::<u64>().ok())
            .ok_or_else(move || DownloadError::InvaildTotal(url))?
    };

    tracing::debug!("File total size is: {total_size}");

    let url = entry.url.clone();
    let mut req = client.get(url);

    if can_resume && entry.allow_resume {
        // 如果已存在的文件大小大于或等于要下载的文件，则重置文件大小，重新下载
        // 因为已经走过一次 chekcusm 了，函数走到这里，则说明肯定文件完整性不对
        if total_size <= file_size {
            tracing::debug!("Exist file size is reset to 0, because total size <= exist file size");
            file_size = 0;
            can_resume = false;
        }

        // 发送 RANGE 的头，传入的是已经下载的文件的大小
        tracing::debug!("oma will set header range as bytes={file_size}-");
        req = req.header(RANGE, format!("bytes={}-", file_size));
    }

    tracing::debug!("Can resume? {can_resume}");

    let resp = req.send().await?;

    if let Err(e) = resp.error_for_status_ref() {
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        match e.status() {
            Some(StatusCode::NOT_FOUND) => {
                let url = entry.url.to_string();
                return Err(DownloadError::NotFound(url));
            }
            _ => return Err(e.into()),
        }
    } else if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    let fpbc = fpb.clone();
    let pb = if let Some(fpb) = fpbc {
        let writer = Writer::default();
        let pb = fpb.mb.add(
            ProgressBar::new(total_size).with_style(oma_console::pb::oma_style_pb(writer, false)?),
        );

        Some(pb)
    } else {
        None
    };

    let progress = if let Some((count, len)) = fpb.clone().and_then(|x| x.progress) {
        format!("({count}/{len}) ")
    } else {
        "".to_string()
    };

    if let Some(pb) = &pb {
        let fpbc = fpb.clone();
        let msg = fpbc.and_then(|x| x.msg).unwrap_or(entry.filename.clone());

        pb.set_message(format!("{progress}{msg}"));
    }

    let mut source = resp;

    // 初始化 checksum 验证器
    // 如果文件存在，则 checksum 验证器已经初试过一次，因此进度条加已经验证过的文件大小

    let hash = entry.hash.clone();
    let mut validator = if let Some(v) = validator {
        if let Some(pb) = &pb {
            pb.inc(file_size);
        }
        Some(v)
    } else if let Some(hash) = hash {
        Some(Checksum::from_sha256_str(&hash)?.get_validator())
    } else {
        None
    };

    let mut dest = if !entry.allow_resume || !can_resume {
        // 如果不能 resume，则加入 truncate 这个 flag，告诉内核截断文件
        // 并把文件长度设置为 0
        tracing::debug!(
            "oma will open file: {} as truncate, create, write and read mode.",
            entry.filename
        );
        let f = tokio::fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .read(true)
            .open(&file)
            .await?;

        tracing::debug!("Setting file length as 0");
        f.set_len(0).await?;

        f
    } else if let Some(dest) = dest {
        tracing::debug!("oma will re use opened dest file for {}", entry.filename);

        dest
    } else {
        tracing::debug!(
            "oma will open file: {} as create, write and read mode.",
            entry.filename
        );

        tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&file)
            .await?
    };

    // 把文件指针移动到末尾
    tracing::debug!("oma will seek file: {} to end", entry.filename);
    dest.seek(SeekFrom::End(0)).await?;

    // 下载！
    tracing::debug!("Start download!");
    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk).await?;
        if let Some(pb) = &pb {
            pb.inc(chunk.len() as u64);
        }

        let fpbc = fpb.clone();
        if let Some(ref gpb) = fpbc.and_then(|x| x.global_bar) {
            gpb.inc(chunk.len() as u64);
        }

        if let Some(ref mut v) = validator {
            v.update(&chunk);
        }
    }

    // 下载完成，告诉内核不再写这个文件了
    tracing::debug!("Download complete! shutting down dest file stream ...");
    dest.shutdown().await?;

    // 最后看看 chekcsum 验证是否通过
    if let Some(v) = validator {
        if !v.finish() {
            tracing::debug!("checksum fail: {}", entry.filename);
            let fpbc = fpb.clone();
            if let Some(ref gpb) = fpbc.and_then(|x| x.global_bar) {
                let pb = pb.unwrap();
                gpb.set_position(gpb.position() - pb.position());
                pb.reset();
            }

            return Err(DownloadError::ChecksumMisMatch(entry.url.to_string()));
        }

        tracing::debug!("checksum success: {}", entry.filename);
    }

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    Ok(true)
}
