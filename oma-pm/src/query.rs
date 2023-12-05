use std::path::Path;

use oma_apt::{
    cache::{Cache, PackageSort},
    package::{Package, Version},
    raw::package::RawPackage,
    records::RecordField,
};
use oma_utils::url_no_escape::url_no_escape;
use tracing::debug;

use crate::{
    pkginfo::PkgInfo,
    search::{search_pkgs, OmaSearchError, SearchResult},
};

#[derive(Debug, thiserror::Error)]
pub enum OmaDatabaseError {
    #[error(transparent)]
    RustApt(#[from] oma_apt::util::Exception),
    #[error("Invaild pattern: {0}")]
    InvaildPattern(String),
    #[error("Can not find package {0} from database")]
    NoPackage(String),
    #[error("Pkg {0} has no version {1}")]
    NoVersion(String, String),
    #[error("Pkg {0} No candidate")]
    NoCandidate(String),
    #[error("Can not find path for local package {0}")]
    NoPath(String),
    #[error(transparent)]
    OmaSearchError(#[from] OmaSearchError),
}

pub struct OmaDatabase<'a> {
    cache: &'a Cache,
}

pub type OmaDatabaseResult<T> = Result<T, OmaDatabaseError>;

impl<'a> OmaDatabase<'a> {
    pub fn new(cache: &'a Cache) -> OmaDatabaseResult<OmaDatabase<'a>> {
        Ok(Self { cache })
    }

    /// Query package from give local file glob
    pub fn query_local_glob(&self, file_glob: &str) -> OmaDatabaseResult<Vec<PkgInfo>> {
        let mut res = vec![];
        let sort = PackageSort::default().only_virtual();

        let glob = self
            .cache
            .packages(&sort)?
            .filter(|x| glob_match::glob_match_with_captures(file_glob, x.name()).is_some())
            .collect::<Vec<_>>();

        for i in glob {
            let real_pkg = real_pkg(&i);
            let pkg = Package::new(self.cache, real_pkg);
            let path = url_no_escape(&format!(
                "file:{}",
                Path::new(i.name())
                    .canonicalize()
                    .map_err(|_| OmaDatabaseError::NoPath(i.name().to_string()))?
                    .to_str()
                    .unwrap_or(pkg.name())
            ))
            .to_string();

            let versions = pkg.versions().collect::<Vec<_>>();

            for ver in &versions {
                let info = PkgInfo::new(ver, &pkg);

                let has = ver.uris().any(|x| url_no_escape(&x) == path);
                if has {
                    res.push(info);
                }
            }
        }

        Ok(res)
    }

    /// Query package from give glob (like: apt*)
    pub fn query_from_glob(
        &self,
        glob: &str,
        filter_candidate: bool,
        select_dbg: bool,
        avail_candidate: bool,
    ) -> OmaDatabaseResult<Vec<PkgInfo>> {
        let mut res = vec![];
        let sort = PackageSort::default().include_virtual();

        let pkgs = self.cache.packages(&sort)?.filter(|x| {
            if glob_match::glob_match_with_captures(glob, x.name()).is_some() {
                debug!("{glob} {}", x.name());
                true
            } else {
                false
            }
        });

        let pkgs = pkgs
            .map(|x| real_pkg(&x))
            .map(|x| Package::new(self.cache, x));

        for pkg in pkgs {
            debug!("Select pkg: {}", pkg.name());
            let versions = pkg.versions().collect::<Vec<_>>();
            for ver in versions {
                let pkginfo = PkgInfo::new(&ver, &pkg);
                let has_dbg = has_dbg(self.cache, &pkg, &ver);

                let is_cand = pkg.candidate().map(|x| x == ver).unwrap_or(false);
                if filter_candidate && is_cand {
                    if !avail_candidate || ver.is_downloadable() {
                        res.push(pkginfo);
                    } else {
                        let ver = pkg.versions().find(|x| x.is_downloadable());

                        if let Some(ver) = ver {
                            res.push(PkgInfo::new(&ver, &pkg));
                        }
                    }
                } else if !filter_candidate {
                    res.push(pkginfo);
                }

                if has_dbg && select_dbg && (is_cand || !filter_candidate) {
                    self.select_dbg(&pkg, &ver, &mut res);
                }
            }
        }

        // 确保数组第一个是 candidate version
        if !filter_candidate {
            let candidate = res.iter().position(|x| {
                Package::new(self.cache, x.raw_pkg.unique())
                    .candidate()
                    .map(|y| y == Version::new(x.version_raw.unique(), self.cache))
                    .unwrap_or(false)
            });

            if let Some(index) = candidate {
                let pkg = res.remove(index);
                res.insert(0, pkg);
            }
        }

        Ok(res)
    }

    /// Query package from give package and version (like: apt=2.5.4)
    pub fn query_from_version(&self, pat: &str, dbg: bool) -> OmaDatabaseResult<Vec<PkgInfo>> {
        let (pkgname, version_str) = pat
            .split_once('=')
            .ok_or_else(|| OmaDatabaseError::InvaildPattern(pat.to_string()))?;

        let pkg = self
            .cache
            .get(pkgname)
            .ok_or_else(|| OmaDatabaseError::NoPackage(pkgname.to_string()))?;

        let version = pkg.get_version(version_str).ok_or_else(|| {
            OmaDatabaseError::NoVersion(pkgname.to_string(), version_str.to_string())
        })?;

        let mut res = vec![];

        let pkginfo = PkgInfo::new(&version, &pkg);
        let has_dbg = has_dbg(self.cache, &pkg, &version);

        res.push(pkginfo);

        if has_dbg && dbg {
            self.select_dbg(&pkg, &version, &mut res);
        }

        Ok(res)
    }

    /// Query package from give package and branch (like: apt/stable)
    pub fn query_from_branch(
        &self,
        pat: &str,
        filter_candidate: bool,
        select_dbg: bool,
    ) -> OmaDatabaseResult<Vec<PkgInfo>> {
        let mut res = vec![];
        let (pkgname, branch) = pat
            .split_once('/')
            .ok_or_else(|| OmaDatabaseError::InvaildPattern(pat.to_string()))?;

        let pkg = self
            .cache
            .get(pkgname)
            .ok_or_else(|| OmaDatabaseError::NoPackage(pkgname.to_string()))?;

        let mut sort = vec![];

        for i in pkg.versions() {
            let item = i.get_record(RecordField::Filename);

            if let Some(item) = item {
                if item.split('/').nth(1) == Some(branch) {
                    sort.push(i)
                }
            }
        }

        sort.sort_by(|x, y| oma_apt::util::cmp_versions(x.version(), y.version()));

        if filter_candidate {
            let version = sort.last();
            if let Some(version) = version {
                let pkginfo = PkgInfo::new(version, &pkg);
                let has_dbg = has_dbg(self.cache, &pkg, version);

                if has_dbg && select_dbg {
                    self.select_dbg(&pkg, version, &mut res);
                }

                res.push(pkginfo);
            }
        } else {
            for i in sort {
                let pkginfo = PkgInfo::new(&i, &pkg);
                let has_dbg = has_dbg(self.cache, &pkg, &i);

                if has_dbg && select_dbg {
                    self.select_dbg(&pkg, &i, &mut res);
                }

                res.push(pkginfo);
            }
        }

        Ok(res)
    }

    /// Smart search pkgs
    pub fn search(&self, keyword: &str) -> OmaDatabaseResult<Vec<SearchResult>> {
        let res = search_pkgs(self.cache, keyword)?;

        Ok(res)
    }

    /// Select -dpg package
    fn select_dbg(&self, pkg: &Package, version: &Version, res: &mut Vec<PkgInfo>) {
        let dbg_pkg_name = format!("{}-dbg", pkg.name());
        let dbg_pkg = self.cache.get(&dbg_pkg_name);
        let version_str = version.version();

        if let Some(dbg_pkg) = dbg_pkg {
            let dbg_ver = dbg_pkg.get_version(version_str);
            if let Some(dbg_ver) = dbg_ver {
                let pkginfo_dbg = PkgInfo::new(&dbg_ver, &dbg_pkg);
                res.push(pkginfo_dbg);
            }
        }
    }

    /// Find mirror candidate and downloadable package version.
    pub fn find_candidate_by_pkgname(&self, pkg: &str) -> OmaDatabaseResult<PkgInfo> {
        if let Some(pkg) = self.cache.get(pkg) {
            // FIXME: candidate 版本不一定是源中能下载的版本
            // 所以要一个个版本遍历直到找到能下载的版本中最高的版本
            for version in pkg.versions() {
                if version.is_downloadable() {
                    let pkginfo = PkgInfo::new(&version, &pkg);
                    debug!(
                        "Pkg: {} selected version: {}",
                        pkg.name(),
                        version.version(),
                    );
                    return Ok(pkginfo);
                }
            }
        }

        Err(OmaDatabaseError::NoCandidate(pkg.to_string()))
    }
}

/// Get real pkg from real pkg or virtual package
fn real_pkg(pkg: &Package) -> RawPackage {
    if !pkg.has_versions() {
        if let Some(provide) = pkg.provides().next() {
            return provide.target_pkg();
        }
    }

    pkg.unique()
}

pub fn has_dbg(cache: &Cache, pkg: &Package<'_>, ver: &Version) -> bool {
    let dbg_pkg = cache.get(&format!("{}-dbg", pkg.name()));

    let has_dbg = if let Some(dbg_pkg) = dbg_pkg {
        dbg_pkg.versions().any(|x| x.version() == ver.version())
    } else {
        false
    };

    has_dbg
}

#[cfg(test)]
mod test {
    use super::OmaDatabase;
    use oma_apt::new_cache;

    #[test]
    fn test_glob_search() {
        let cache = new_cache!().unwrap();
        let db = OmaDatabase::new(&cache).unwrap();
        let res_filter = db.query_from_glob("apt*", true, false, false).unwrap();
        let res = db.query_from_glob("apt*", false, false, false).unwrap();

        for i in res_filter {
            i.print_info(&cache);
        }

        println!("---\n");

        for i in res {
            i.print_info(&cache);
        }
    }

    #[test]
    fn test_virtual_pkg_search() {
        let cache = new_cache!().unwrap();
        let db = OmaDatabase::new(&cache).unwrap();
        let res_filter = db.query_from_glob("telegram", true, false, false).unwrap();

        for i in res_filter {
            i.print_info(&cache);
        }
    }

    #[test]
    fn test_branch_search() {
        let cache = new_cache!().unwrap();
        let db = OmaDatabase::new(&cache).unwrap();
        let res_filter = db.query_from_branch("apt/stable", true, false).unwrap();

        for i in res_filter {
            i.print_info(&cache);
        }
    }
}
