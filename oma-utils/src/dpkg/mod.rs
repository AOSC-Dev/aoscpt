use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use tracing::debug;

#[derive(Debug, thiserror::Error)]
pub enum DpkgError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("`dpkg' returned an error: {0}")]
    DpkgRunError(i32),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Failed to query dpkg database")]
    FailedToQueryDpkgDatabase,
}

/// Get architecture from dpkg
pub fn dpkg_arch<P: AsRef<Path>>(sysroot: P) -> Result<String, DpkgError> {
    let dpkg = Command::new("dpkg")
        .arg("--root")
        .arg(sysroot.as_ref().display().to_string())
        .arg("--print-architecture")
        .output()?;

    if !dpkg.status.success() {
        return Err(DpkgError::DpkgRunError(dpkg.status.code().unwrap_or(1)));
    }

    let output = std::str::from_utf8(&dpkg.stdout)?.trim().to_string();

    Ok(output)
}

pub fn is_hold<P: AsRef<Path>>(pkg: &str, sysroot: P) -> Result<bool, DpkgError> {
    let list = get_selections(sysroot)?;

    let status = list
        .iter()
        .find(|(x, _)| x == pkg)
        .map(|x| x.1 == "hold")
        .unwrap_or(false);

    Ok(status)
}

/// Mark hold/unhold status use dpkg --set-selections
pub fn mark_version_status<P: AsRef<Path>>(
    pkgs: &[String],
    hold: bool,
    dry_run: bool,
    sysroot: P,
) -> Result<Vec<(&str, bool)>, DpkgError> {
    let list = get_selections(&sysroot)?;

    let mut res = vec![];

    for pkg in pkgs {
        if list.iter().any(|(x, status)| {
            x == pkg
                && match hold {
                    true => status == "hold",
                    false => status == "install",
                }
        }) {
            debug!("pkg {pkg} set to hold = {hold} is set = false");
            res.push((pkg.as_str(), false));
            continue;
        }

        debug!("pkg {pkg} set to hold = {hold} is set = true");

        if dry_run {
            continue;
        }

        let mut dpkg = Command::new("dpkg")
            .arg(sysroot.as_ref().display().to_string())
            .arg("--set-selections")
            .stdin(Stdio::piped())
            .spawn()?;

        let op = match hold {
            true => "hold",
            false => "install",
        };

        dpkg.stdin
            .as_mut()
            .unwrap()
            .write_all(format!("{pkg} {op}").as_bytes())?;

        dpkg.wait()?;

        res.push((pkg.as_str(), true));
    }

    Ok(res)
}

fn get_selections<P: AsRef<Path>>(sysroot: P) -> Result<Vec<(String, String)>, DpkgError> {
    let dpkg = Command::new("dpkg")
        .arg(sysroot.as_ref().display().to_string())
        .arg("--get-selections")
        .output()?;
    if !dpkg.status.success() {
        return Err(DpkgError::DpkgRunError(dpkg.status.code().unwrap_or(1)));
    }

    let mut selections = std::str::from_utf8(&dpkg.stdout)?.split('\n');
    selections.nth_back(0);

    let list = Some(())
        .and_then(|_| {
            let mut list = vec![];
            for i in selections {
                let mut split = i.split_whitespace();
                let name = split.next()?;
                let status = split.next()?;

                list.push((name.to_string(), status.to_string()));
            }

            Some(list)
        })
        .ok_or_else(|| DpkgError::FailedToQueryDpkgDatabase)?;

    Ok(list)
}
