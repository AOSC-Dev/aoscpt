use std::path::PathBuf;

use clap::Args;
use oma_history::SummaryType;
use oma_pm::apt::{AptConfig, OmaApt, OmaAptArgs};

use crate::{
    config::Config,
    error::OutputError,
    utils::{dbus_check, root},
};

use super::utils::{auth_config, lock_oma, no_check_dbus_warn, CommitChanges};
use crate::args::CliExecuter;

#[derive(Debug, Args)]
pub struct FixBroken {
    /// Do not fix dpkg broken status
    #[arg(short, long)]
    no_fix_dpkg_status: bool,
    /// Install package(s) without fsync(2)
    #[arg(long)]
    force_unsafe_io: bool,
    /// Ignore repository and package dependency issues
    #[arg(long)]
    force_yes: bool,
    /// Replace configuration file(s) in the system those shipped in the package(s) to be installed (invokes `dpkg --force-confnew`)
    #[arg(long)]
    force_confnew: bool,
    /// Auto remove unnecessary package(s)
    #[arg(long)]
    autoremove: bool,
    /// Remove package(s) also remove configuration file(s), like apt purge
    #[arg(long, visible_alias = "purge")]
    remove_config: bool,
    /// Run oma in “dry-run” mode. Useful for testing changes and operations without making changes to the system
    #[arg(from_global)]
    dry_run: bool,
    /// Run oma do not check dbus
    #[arg(from_global)]
    no_check_dbus: bool,
    /// Set sysroot target directory
    #[arg(from_global)]
    sysroot: PathBuf,
    /// Set apt options
    #[arg(from_global)]
    apt_options: Vec<String>,
}

impl CliExecuter for FixBroken {
    fn execute(self, config: &Config, no_progress: bool) -> Result<i32, OutputError> {
        root()?;
        lock_oma()?;

        let FixBroken {
            force_unsafe_io,
            force_yes,
            force_confnew,
            autoremove,
            remove_config,
            dry_run,
            no_check_dbus,
            sysroot,
            apt_options,
            no_fix_dpkg_status,
        } = self;

        let mut _fds = None;

        if !no_check_dbus && !config.no_check_dbus() && !dry_run {
            _fds = Some(dbus_check(false)?);
        } else {
            no_check_dbus_warn();
        }

        let auth_config = auth_config(&sysroot);
        let auth_config = auth_config.as_ref();

        let oma_apt_args = OmaAptArgs::builder()
            .sysroot(sysroot.to_string_lossy().to_string())
            .dpkg_force_unsafe_io(force_unsafe_io)
            .force_yes(force_yes)
            .dpkg_force_confnew(force_confnew)
            .another_apt_options(apt_options)
            .build();
        let apt = OmaApt::new(vec![], oma_apt_args, dry_run, AptConfig::new())?;

        CommitChanges::builder()
            .apt(apt)
            .dry_run(dry_run)
            .request_type(SummaryType::FixBroken)
            .no_fixbroken(false)
            .no_progress(no_progress)
            .sysroot(sysroot.to_string_lossy().to_string())
            .fix_dpkg_status(!no_fix_dpkg_status)
            .protect_essential(config.protect_essentials())
            .yes(false)
            .autoremove(autoremove)
            .remove_config(remove_config)
            .maybe_auth_config(auth_config)
            .network_thread(config.network_thread())
            .build()
            .run()
    }
}
