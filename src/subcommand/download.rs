use std::path::PathBuf;
use std::thread;

use clap::Args;
use flume::unbounded;
use oma_pm::apt::{AptConfig, DownloadConfig, OmaApt, OmaAptArgs};
use oma_pm::matches::PackagesMatcher;
use reqwest::StatusCode;
use tracing::{error, info};

use crate::config::Config;
use crate::pb::{NoProgressBar, OmaMultiProgressBar, RenderDownloadProgress};
use crate::utils::is_root;
use crate::{due_to, fl, success, HTTP_CLIENT};
use crate::{error::OutputError, subcommand::utils::handle_no_result};

use crate::args::CliExecuter;

use super::utils::{auth_config, is_terminal};

#[derive(Debug, Args)]
pub struct Download {
    /// Package(s) to download
    #[arg(required = true)]
    packages: Vec<String>,
    /// The path where package(s) should be downloaded to
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    /// Run oma in “dry-run” mode. Useful for testing changes and operations without making changes to the system
    #[arg(from_global)]
    dry_run: bool,
}

impl CliExecuter for Download {
    fn execute(self, config: &Config, no_progress: bool) -> Result<i32, OutputError> {
        let Download {
            packages,
            path,
            dry_run,
        } = self;

        let path = path.canonicalize().map_err(|e| OutputError {
            description: format!("Failed to canonicalize path: {}", path.display()),
            source: Some(Box::new(e)),
        })?;

        let apt_config = AptConfig::new();
        let oma_apt_args = OmaAptArgs::builder().build();
        let apt = OmaApt::new(vec![], oma_apt_args, dry_run, apt_config)?;
        let matcher = PackagesMatcher::builder()
            .cache(&apt.cache)
            .filter_candidate(true)
            .filter_downloadable_candidate(true)
            .select_dbg(false)
            .build();

        let (pkgs, no_result) =
            matcher.match_pkgs_and_versions(packages.iter().map(|x| x.as_str()))?;
        handle_no_result("/", no_result, no_progress)?;

        let (tx, rx) = unbounded();

        thread::spawn(move || {
            let mut pb: Box<dyn RenderDownloadProgress> = if no_progress || !is_terminal() {
                Box::new(NoProgressBar::default())
            } else {
                Box::new(OmaMultiProgressBar::default())
            };
            pb.render_progress(&rx);
        });

        let (success, failed) = apt.download(
            &HTTP_CLIENT,
            pkgs,
            DownloadConfig {
                network_thread: Some(config.network_thread()),
                download_dir: Some(&path),
                auth: auth_config("/").as_ref(),
            },
            dry_run,
            |event| async {
                if let Err(e) = tx.send_async(event).await {
                    error!("{}", e);
                }
            },
        )?;

        if !success.is_empty() {
            success!(
                "{}",
                fl!(
                    "successfully-download-to-path",
                    len = success.len(),
                    path = path.display().to_string()
                )
            );
        }

        if !failed.is_empty() {
            let len = failed.len();
            for f in failed {
                let e = OutputError::from(f);

                error!("{e}");
                if let Some(s) = e.source {
                    due_to!("{s}");
                    if let Some(e) = s.downcast_ref::<reqwest::Error>() {
                        if e.status().is_some_and(|x| x == StatusCode::UNAUTHORIZED) {
                            if !is_root() {
                                info!("{}", fl!("auth-need-permission"));
                            } else {
                                info!("{}", fl!("lack-auth-config-1"));
                                info!("{}", fl!("lack-auth-config-2"));
                            }
                        }
                    }
                }
            }

            return Err(OutputError {
                description: fl!("download-failed-with-len", len = len),
                source: None,
            });
        }

        Ok(0)
    }
}
