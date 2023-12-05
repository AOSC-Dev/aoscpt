use chrono::Local;
use oma_console::success;
use oma_history::connect_or_create_db;
use oma_history::write_history_entry;
use oma_history::SummaryType;
use oma_pm::apt::AptArgsBuilder;
use oma_pm::apt::OmaApt;
use oma_pm::apt::OmaAptArgsBuilder;
use oma_pm::apt::OmaAptError;
use tracing::info;
use tracing::warn;

use crate::error::OutputError;
use crate::fl;
use crate::pb;
use crate::table::table_for_install_pending;
use crate::utils::create_async_runtime;
use crate::utils::dbus_check;
use crate::utils::multibar;
use crate::utils::root;
use crate::UpgradeArgs;

use super::utils::check_empty_op;
use super::utils::handle_event_without_progressbar;
use super::utils::handle_no_result;
use super::utils::refresh;

pub fn execute(
    pkgs_unparse: Vec<String>,
    args: UpgradeArgs,
    dry_run: bool,
    no_progress: bool,
    download_pure_db: bool,
) -> Result<i32, OutputError> {
    root()?;

    let rt = create_async_runtime()?;
    dbus_check(&rt)?;

    refresh(dry_run, no_progress, download_pure_db, &args.sysroot)?;

    if args.yes {
        warn!("{}", fl!("automatic-mode-warn"));
    }

    let local_debs = pkgs_unparse
        .iter()
        .filter(|x| x.ends_with(".deb"))
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let pkgs_unparse = pkgs_unparse.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let mut retry_times = 1;

    let apt_args = AptArgsBuilder::default()
        .dpkg_force_all(args.dpkg_force_all)
        .dpkg_force_confnew(args.force_confnew)
        .force_yes(args.force_yes)
        .yes(args.yes)
        .no_progress(no_progress)
        .build()?;

    let oma_apt_args = OmaAptArgsBuilder::default()
        .sysroot(args.sysroot.clone())
        .build()?;

    loop {
        let mut apt = OmaApt::new(local_debs.clone(), oma_apt_args.clone(), dry_run)?;
        apt.upgrade()?;

        let (pkgs, no_result) = apt.select_pkg(&pkgs_unparse, false, true, false)?;
        handle_no_result(no_result);

        apt.install(&pkgs, false)?;

        let op = apt.summary()?;
        let op_after = op.clone();

        let install = op.install;
        let remove = op.remove;
        let disk_size = op.disk_size;

        if check_empty_op(&install, &remove) {
            return Ok(0);
        }

        apt.resolve(false)?;
        apt.check_disk_size()?;

        if retry_times == 1 {
            table_for_install_pending(&install, &remove, &disk_size, !args.yes, dry_run)?;
        }

        let start_time = Local::now().timestamp();

        let (mb, pb_map, global_is_set) = multibar();
        match apt.commit(None, &apt_args, |count, event, total| {
            if !no_progress {
                pb!(event, mb, pb_map, count, total, global_is_set)
            } else {
                handle_event_without_progressbar(event);
            }
        }) {
            Ok(()) => {
                write_history_entry(
                    op_after,
                    SummaryType::Upgrade(
                        pkgs.iter()
                            .map(|x| format!("{} {}", x.raw_pkg.name(), x.version_raw.version()))
                            .collect::<Vec<_>>(),
                    ),
                    connect_or_create_db(true, args.sysroot)?,
                    dry_run,
                    start_time,
                    true,
                )?;
                success!("{}", fl!("history-tips-1"));
                info!("{}", fl!("history-tips-2"));
                return Ok(0);
            }
            Err(e) => match e {
                OmaAptError::RustApt(_) => {
                    if retry_times == 3 {
                        write_history_entry(
                            op_after,
                            SummaryType::Upgrade(
                                pkgs.iter()
                                    .map(|x| {
                                        format!("{} {}", x.raw_pkg.name(), x.version_raw.version())
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                            connect_or_create_db(true, args.sysroot)?,
                            dry_run,
                            start_time,
                            false,
                        )?;
                        info!("{}", fl!("history-tips-2"));
                        return Err(OutputError::from(e));
                    }
                    warn!("{e}, retrying ...");
                    retry_times += 1;
                }
                _ => return Err(OutputError::from(e)),
            },
        }
    }
}
