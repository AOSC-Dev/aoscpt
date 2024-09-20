use std::borrow::Cow;

use oma_console::{indicatif::ProgressBar, pager::Pager, pb::spinner_style};
use oma_pm::{
    apt::{AptConfig, OmaApt, OmaAptArgs},
    query::{OmaDatabase, SearchEngine},
};
use tracing::warn;

use crate::{error::OutputError, table::oma_display_with_normal_output};
use crate::{fl, utils::SearchResultDisplay};

pub fn execute(
    args: &[String],
    no_progress: bool,
    sysroot: String,
    engine: Cow<String>,
    no_pager: bool,
) -> Result<i32, OutputError> {
    let oma_apt_args = OmaAptArgs::builder().sysroot(sysroot).build();
    let apt = OmaApt::new(vec![], oma_apt_args, false, AptConfig::new())?;
    let db = OmaDatabase::new(&apt.cache)?;
    let s = args.concat();

    let (sty, inv) = spinner_style();

    let pb = if !no_progress {
        let pb = ProgressBar::new_spinner().with_style(sty);
        pb.enable_steady_tick(inv);
        pb.set_message(fl!("searching"));

        Some(pb)
    } else {
        None
    };

    let res = db.search(
        &s,
        match engine.as_str() {
            "indicium" => SearchEngine::Indicium(Box::new(|_| {})),
            "strsim" => SearchEngine::Strsim,
            "text" => SearchEngine::Text,
            x => {
                warn!("Unsupport mode: {x}, fallback to indicium ...");
                SearchEngine::Indicium(Box::new(|_| {}))
            }
        },
    )?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    let mut pager = if !no_pager {
        oma_display_with_normal_output(false, res.len() * 2)?
    } else {
        Pager::plain()
    };

    let mut writer = pager.get_writer().map_err(|e| OutputError {
        description: "Failed to get writer".to_string(),
        source: Some(Box::new(e)),
    })?;

    for i in res {
        write!(writer, "{}", SearchResultDisplay(&i)).ok();
    }

    drop(writer);
    let exit = pager.wait_for_exit().map_err(|e| OutputError {
        description: "Failed to wait exit".to_string(),
        source: Some(Box::new(e)),
    })?;

    Ok(exit.into())
}
