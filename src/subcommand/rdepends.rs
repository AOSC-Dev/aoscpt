use std::io::stdout;

use oma_pm::{
    apt::{AptConfig, OmaApt, OmaAptArgs},
    matches::PackagesMatcher,
};
use oma_utils::dpkg::dpkg_arch;
use std::io::Write;

use crate::error::OutputError;

use super::utils::{check_unsupported_stmt, handle_no_result};

pub fn execute(
    pkgs: Vec<String>,
    sysroot: String,
    json: bool,
    another_apt_options: Vec<String>,
    no_progress: bool,
) -> Result<i32, OutputError> {
    for pkg in &pkgs {
        check_unsupported_stmt(pkg);
    }

    let oma_apt_args = OmaAptArgs::builder()
        .sysroot(sysroot.clone())
        .another_apt_options(another_apt_options)
        .build();

    let apt = OmaApt::new(vec![], oma_apt_args, false, AptConfig::new())?;

    let arch = dpkg_arch(&sysroot)?;
    let matcher = PackagesMatcher::builder()
        .cache(&apt.cache)
        .native_arch(&arch)
        .build();

    let (pkgs, no_result) = matcher.match_pkgs_and_versions(pkgs.iter().map(|x| x.as_str()))?;

    handle_no_result(sysroot, no_result, no_progress)?;

    if !json {
        for pkg in pkgs {
            println!("{}:", pkg.raw_pkg.fullname(true));
            println!("  Reverse dependencies:");
            let all_deps = pkg.get_rdeps(&apt.cache)?;

            for (k, v) in all_deps {
                let mut uniq_dep_src = vec![];
                for dep in v.inner() {
                    for b_dep in dep {
                        if let (Some(symbol), Some(ver)) = (b_dep.comp_symbol, b_dep.target_ver) {
                            uniq_dep_src.push((
                                b_dep.name,
                                format!("{} {symbol} {ver}", pkg.raw_pkg.fullname(true)),
                            ));
                        } else {
                            uniq_dep_src.push((b_dep.name, String::new()));
                        };
                    }
                }

                uniq_dep_src.sort();
                uniq_dep_src.dedup();
                for (name, src) in uniq_dep_src {
                    if src.is_empty() {
                        println!("    {k}: {}", name)
                    } else {
                        println!("    {k}: {} ({})", name, src);
                    }
                }
            }
        }
    } else {
        let mut stdout = stdout();
        for pkg in pkgs {
            writeln!(
                stdout,
                "{}",
                serde_json::json!({
                    "name": pkg.raw_pkg.fullname(true),
                    "rdeps": pkg.get_rdeps(&apt.cache)?,
                })
            )
            .ok();
        }
    }

    Ok(0)
}
