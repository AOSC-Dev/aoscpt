use oma_history::SummaryType;
use oma_pm::apt::{AptConfig, OmaApt, OmaAptArgs};
use reqwest::Client;

use crate::{
    error::OutputError,
    utils::{dbus_check, root},
    OmaArgs,
};

use super::utils::{lock_oma, no_check_dbus_warn, CommitRequest};

pub fn execute(oma_args: OmaArgs, sysroot: String, client: Client) -> Result<i32, OutputError> {
    root()?;
    lock_oma()?;

    let OmaArgs {
        dry_run,
        network_thread,
        no_progress,
        no_check_dbus,
        protect_essentials: protect_essential,
    } = oma_args;

    let mut fds = None;

    if !no_check_dbus {
        fds = Some(dbus_check(false)?);
    } else {
        no_check_dbus_warn();
    }

    let oma_apt_args = OmaAptArgs::builder()
        .sysroot(sysroot.clone())
        .no_progress(no_progress)
        .build();
    let apt = OmaApt::new(vec![], oma_apt_args, dry_run, AptConfig::new())?;

    let request = CommitRequest {
        apt,
        dry_run,
        request_type: SummaryType::FixBroken,
        no_fixbroken: false,
        network_thread,
        no_progress,
        sysroot,
        fix_dpkg_status: false,
        protect_essential,
        client: &client,
        yes: false,
    };

    let code = request.run()?;

    drop(fds);

    Ok(code)
}
