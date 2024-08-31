use anyhow::anyhow;
use chrono::{Local, LocalResult, TimeZone};
use dialoguer::{theme::ColorfulTheme, Select};
use oma_history::{
    connect_db, find_history_by_id, list_history, HistoryListEntry, SummaryType, DATABASE_PATH,
};
use oma_pm::apt::{AptConfig, InstallOperation};
use oma_pm::pkginfo::PtrIsNone;
use oma_pm::{
    apt::{AptArgsBuilder, FilterMode, OmaApt, OmaAptArgsBuilder},
    pkginfo::PkgInfo,
};
use reqwest::Client;

use std::path::Path;
use std::{borrow::Cow, sync::atomic::Ordering};

use crate::OmaArgs;
use crate::{
    error::OutputError,
    table::table_for_history_pending,
    utils::{create_async_runtime, dbus_check, root},
    ALLOWCTRLC,
};

use super::utils::{
    handle_no_result, lock_oma, no_check_dbus_warn, normal_commit, NormalCommitArgs,
};

pub fn execute_history(sysroot: String) -> Result<i32, OutputError> {
    let conn = connect_db(Path::new(&sysroot).join(DATABASE_PATH), false)?;

    let list = list_history(&conn)?;
    let display_list = format_summary_log(&list, false)
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<_>>();

    ALLOWCTRLC.store(true, Ordering::Relaxed);

    let mut old_selected = 0;

    loop {
        let selected =
            dialoguer_select_history(&display_list, old_selected).map_err(|_| anyhow!(""))?;
        old_selected = selected;

        let selected = &list[selected];
        let id = selected.id;
        let op = find_history_by_id(&conn, id)?;
        let install = &op.install;
        let remove = &op.remove;
        let disk_size = &op.disk_size;

        table_for_history_pending(install, remove, disk_size)?;
    }
}

pub fn execute_undo(
    oma_args: OmaArgs,
    sysroot: String,
    client: &Client,
) -> Result<i32, OutputError> {
    root()?;
    lock_oma()?;

    let OmaArgs {
        dry_run: _,
        network_thread,
        no_progress,
        no_check_dbus,
        protect_essentials: protect_essential,
        ..
    } = oma_args;

    let fds = if !no_check_dbus {
        let rt = create_async_runtime()?;
        Some(dbus_check(&rt, false)?)
    } else {
        no_check_dbus_warn();
        None
    };

    let conn = connect_db(Path::new(&sysroot).join(DATABASE_PATH), false)?;

    let list = list_history(&conn)?;
    let display_list = format_summary_log(&list, true);
    let selected = dialoguer_select_history(
        &display_list
            .clone()
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<_>>(),
        0,
    )?;

    let selected = &list[display_list[selected].1];
    let id = selected.id;
    let op = find_history_by_id(&conn, id)?;

    let oma_apt_args = OmaAptArgsBuilder::default()
        .sysroot(sysroot.clone())
        .build()?;
    let mut apt = OmaApt::new(vec![], oma_apt_args, false, AptConfig::new())?;

    let mut delete = vec![];
    let mut install = vec![];

    if !op.install.is_empty() {
        for i in &op.install {
            match i.op() {
                InstallOperation::Default | InstallOperation::Download => unreachable!(),
                InstallOperation::Install => delete.push(i.name()),
                InstallOperation::ReInstall => continue,
                InstallOperation::Upgrade => install.push((i.name(), i.old_version().unwrap())),
                InstallOperation::Downgrade => install.push((i.name(), i.old_version().unwrap())),
            }
        }
    }

    if !op.remove.is_empty() {
        for i in &op.remove {
            if let Some(ver) = i.version() {
                install.push((i.name(), ver));
            }
        }
    }

    let (delete, no_result) = apt.select_pkg(&delete, false, true, false)?;
    handle_no_result(no_result)?;

    apt.remove(&delete, false, true)?;

    let pkgs = apt.filter_pkgs(&[FilterMode::Default])?.collect::<Vec<_>>();

    let install = install
        .iter()
        .filter_map(|(pkg, ver)| {
            let pkg = pkgs.iter().find(move |y| &y.name() == pkg);

            if let Some(pkg) = pkg {
                Some((pkg, pkg.get_version(ver)?))
            } else {
                None
            }
        })
        .map(|(x, y)| PkgInfo::new(&y, x))
        .collect::<Result<Vec<PkgInfo>, PtrIsNone>>()
        .map_err(|e| OutputError {
            description: e.to_string(),
            source: None,
        })?;

    apt.install(&install, false)?;

    let args = NormalCommitArgs {
        apt,
        dry_run: false,
        typ: SummaryType::Undo,
        apt_args: AptArgsBuilder::default().no_progress(no_progress).build()?,
        no_fixbroken: false,
        network_thread,
        no_progress,
        sysroot,
        fix_dpkg_status: true,
        protect_essential,
    };

    normal_commit(args, client)?;

    drop(fds);

    Ok(0)
}

fn dialoguer_select_history(
    display_list: &[String],
    old_selected: usize,
) -> Result<usize, OutputError> {
    let selected = Select::with_theme(&ColorfulTheme::default())
        .items(display_list)
        .default(old_selected)
        .interact()
        .map_err(|_| anyhow!(""))?;

    Ok(selected)
}

fn format_summary_log(list: &[HistoryListEntry], undo: bool) -> Vec<(String, usize)> {
    let display_list = list
        .iter()
        .enumerate()
        .filter(|(_, log)| {
            if undo {
                log.t != SummaryType::FixBroken && log.t != SummaryType::Undo
            } else {
                true
            }
        })
        .map(|(index, log)| {
            let date = format_date(log.time);
            let s = match &log.t {
                SummaryType::Install(v) if v.len() > 3 => format!(
                    "{}Installed {} ... (and {} more) [{}]",
                    format_success(log.is_success),
                    v[..3].join(" "),
                    v.len() - 3,
                    date
                ),
                SummaryType::Install(v) => format!(
                    "{}Installed {} [{date}]",
                    format_success(log.is_success),
                    v.join(" "),
                ),
                SummaryType::Upgrade(v) if v.is_empty() => {
                    format!("Upgraded system [{date}]")
                }
                SummaryType::Upgrade(v) if v.len() > 3 => format!(
                    "{}Upgraded system and installed {}... (and {} more) [{date}]",
                    format_success(log.is_success),
                    v[..3].join(" "),
                    v.len() - 3
                ),
                SummaryType::Upgrade(v) => format!(
                    "{}Upgraded system and install {} [{date}]",
                    format_success(log.is_success),
                    v.join(" "),
                ),
                SummaryType::Remove(v) if v.len() > 3 => format!(
                    "{}Removed {} ... (and {} more)",
                    format_success(log.is_success),
                    v[..3].join(" "),
                    v.len() - 3
                ),
                SummaryType::Remove(v) => format!("Removed {} [{date}]", v.join(" ")),
                SummaryType::FixBroken => format!("Attempted to fix broken dependencies [{date}]"),
                SummaryType::TopicsChanged { add, remove } if remove.is_empty() => {
                    format!(
                        "{}Topics changed: enabled {}{} [{date}]",
                        format_success(log.is_success),
                        if add.len() <= 3 {
                            add.join(" ")
                        } else {
                            add[..3].join(" ")
                        },
                        if add.len() <= 3 {
                            Cow::Borrowed("")
                        } else {
                            Cow::Owned(format!(" ... (and {} more)", add.len() - 3))
                        }
                    )
                }
                SummaryType::TopicsChanged { add, remove } if add.is_empty() => {
                    format!(
                        "{}Topics changed: disabled {}{} [{date}]",
                        format_success(log.is_success),
                        if remove.len() <= 3 {
                            add.join(" ")
                        } else {
                            remove[..3].join(" ")
                        },
                        if remove.len() <= 3 {
                            Cow::Borrowed("")
                        } else {
                            Cow::Owned(format!(" ... (and {} more)", remove.len() - 3))
                        }
                    )
                }
                SummaryType::TopicsChanged { add, remove } => {
                    format!(
                        "{}Topics changed: enabled {}{}, disabled {}{} [{date}]",
                        format_success(log.is_success),
                        if add.len() <= 3 {
                            add.join(" ")
                        } else {
                            add[..3].join(" ")
                        },
                        if add.len() <= 3 {
                            Cow::Borrowed("")
                        } else {
                            Cow::Owned(format!(" ... (and {} more)", add.len() - 3))
                        },
                        if remove.len() <= 3 {
                            remove.join(" ")
                        } else {
                            remove[..3].join(" ")
                        },
                        if remove.len() <= 3 {
                            Cow::Borrowed("")
                        } else {
                            Cow::Owned(format!(" ... (and {} more)", add.len() - 3))
                        },
                    )
                }
                SummaryType::Undo => format!("Undone [{date}]"),
                SummaryType::Changes => "Change packages".to_string(),
            };
            (s, index)
        })
        .collect::<Vec<_>>();

    display_list
}

fn format_date(date: i64) -> String {
    let dt = match Local.timestamp_opt(date, 0) {
        LocalResult::None => Local.timestamp_opt(0, 0).unwrap(),
        x => x.unwrap(),
    };

    let s = dt.format("%H:%M:%S on %Y-%m-%d").to_string();

    s
}

fn format_success(is_success: bool) -> &'static str {
    if is_success {
        ""
    } else {
        "[FAIL] "
    }
}
