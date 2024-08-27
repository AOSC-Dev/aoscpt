use std::error::Error;
use std::io::stdout;

use oma_console::due_to;
use oma_console::print::Action;
use oma_contents::searcher::{pure_search, ripgrep_search, Mode};
use oma_contents::OmaContentsError;
use oma_pm::apt::{OmaApt, OmaAptArgsBuilder};
use oma_pm::format_description;
use tracing::error;

use crate::error::OutputError;
use crate::table::PagerPrinter;
use crate::{color_formatter, fl};

const FILTER_JARO_NUM: u8 = 204;
const APT_LIST_PATH: &str = "/var/lib/apt/lists";

type IndexSet<T> = indexmap::IndexSet<T, ahash::RandomState>;

pub fn execute(query: &str) -> Result<i32, OutputError> {
    let mut res = IndexSet::with_hasher(ahash::RandomState::new());

    let cb = |line| {
        if !res.contains(&line) {
            res.insert(line);
        }
    };

    let search_res = if which::which("rg").is_ok() {
        ripgrep_search(APT_LIST_PATH, Mode::BinProvides, query, cb)
    } else {
        pure_search(APT_LIST_PATH, Mode::BinProvides, query, cb)
    };

    match search_res {
        Ok(()) if res.is_empty() => {
            error!("{}", fl!("command-not-found", kw = query));
        }
        Ok(()) => {
            let oma_apt_args = OmaAptArgsBuilder::default().build()?;
            let apt = OmaApt::new(vec![], oma_apt_args, false)?;

            let mut jaro = jaro_nums(res, query);

            let all_match = jaro
                .iter()
                .filter(|x| x.2 == u8::MAX)
                .map(|x| x.to_owned())
                .collect::<Vec<_>>();

            if !all_match.is_empty() {
                jaro = all_match;
            }

            let mut res = vec![];

            for (pkg, file, jaro) in jaro {
                if jaro < FILTER_JARO_NUM {
                    continue;
                }

                if let Some(pkg) = apt.cache.get(&pkg) {
                    let desc = pkg
                        .candidate()
                        .and_then(|x| {
                            x.description()
                                .map(|x| format_description(&x).0.to_string())
                        })
                        .unwrap_or_else(|| "no description.".to_string());

                    let entry = (
                        color_formatter()
                            .color_str(pkg.name(), Action::Emphasis)
                            .bold()
                            .to_string(),
                        color_formatter()
                            .color_str(file, Action::Secondary)
                            .to_string(),
                        desc,
                    );

                    res.push(entry);
                }
            }

            if res.is_empty() {
                error!("{}", fl!("command-not-found", kw = query));
            } else {
                println!("{}\n", fl!("command-not-found-with-result", kw = query));
                let mut printer = PagerPrinter::new(stdout());
                printer
                    .print_table(res, vec!["Name", "Path", "Description"])
                    .ok();
            }
        }
        Err(e) => {
            if let OmaContentsError::NoResult = e {
                error!("{}", fl!("command-not-found", kw = query));
            } else {
                let err = OutputError::from(e);
                if !err.to_string().is_empty() {
                    error!("{err}");
                    if let Some(source) = err.source() {
                        due_to!("{source}");
                    }
                }
            }
        }
    }

    Ok(127)
}

fn jaro_nums(input: IndexSet<(String, String)>, query: &str) -> Vec<(String, String, u8)> {
    let mut output = vec![];

    for (pkg, file) in input {
        if pkg == query {
            output.push((pkg, file, u8::MAX));
            continue;
        }

        let binary_name = file.split('/').next_back().unwrap_or(&file);

        if binary_name == query {
            output.push((pkg, file, u8::MAX));
            continue;
        }

        let num = (strsim::jaro_winkler(query, binary_name) * 255.0) as u8;

        output.push((pkg, file, num));
    }

    output.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    output
}
