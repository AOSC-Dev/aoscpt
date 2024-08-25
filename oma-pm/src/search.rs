use ahash::AHashMap;
use cxx::UniquePtr;
use indicium::simple::{Indexable, SearchIndex};
use oma_apt::{
    cache::{Cache, PackageSort},
    error::{AptError, AptErrors},
    raw::{IntoRawIter, PkgIterator},
    Package,
};
use std::{collections::hash_map::Entry, fmt::Debug};

use crate::{format_description, pkginfo::PtrIsNone, query::has_dbg};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PackageStatus {
    Avail,
    Installed,
    Upgrade,
}

impl PartialOrd for PackageStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PackageStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            PackageStatus::Avail => match other {
                PackageStatus::Avail => std::cmp::Ordering::Equal,
                PackageStatus::Installed => std::cmp::Ordering::Greater,
                PackageStatus::Upgrade => std::cmp::Ordering::Less,
            },
            PackageStatus::Installed => match other {
                PackageStatus::Avail => std::cmp::Ordering::Less,
                PackageStatus::Installed => std::cmp::Ordering::Equal,
                PackageStatus::Upgrade => std::cmp::Ordering::Less,
            },
            PackageStatus::Upgrade => match other {
                PackageStatus::Avail => std::cmp::Ordering::Greater,
                PackageStatus::Installed => std::cmp::Ordering::Greater,
                PackageStatus::Upgrade => std::cmp::Ordering::Equal,
            },
        }
    }
}

pub struct SearchEntry {
    name: String,
    description: String,
    status: PackageStatus,
    provides: Vec<String>,
    has_dbg: bool,
    raw_pkg: UniquePtr<PkgIterator>,
    section_is_base: bool,
}

impl Debug for SearchEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchEntry")
            .field("pkgname", &self.name)
            .field("description", &self.description)
            .field("status", &self.status)
            .field("provides", &self.provides)
            .field("has_dbg", &self.has_dbg)
            .field("raw_pkg", &self.raw_pkg.name())
            .field("section_is_base", &self.section_is_base)
            .finish()
    }
}

impl Indexable for SearchEntry {
    fn strings(&self) -> Vec<String> {
        let mut v = vec![self.name.clone(), self.description.clone()];
        let provides = self.provides.clone().into_iter();
        v.extend(provides);
        v
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OmaSearchError {
    #[error(transparent)]
    AptErrors(#[from] AptErrors),
    #[error(transparent)]
    AptError(#[from] AptError),
    #[error(transparent)]
    AptCxxException(#[from] cxx::Exception),
    #[error("No result found: {0}")]
    NoResult(String),
    #[error("Failed to get candidate version: {0}")]
    FailedGetCandidate(String),
    #[error(transparent)]
    PtrIsNone(#[from] PtrIsNone),
}

pub type OmaSearchResult<T> = Result<T, OmaSearchError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub name: String,
    pub desc: String,
    pub old_version: Option<String>,
    pub new_version: String,
    pub full_match: bool,
    pub dbg_package: bool,
    pub status: PackageStatus,
    pub is_base: bool,
}

pub struct OmaSearch<'a> {
    cache: &'a Cache,
    pkg_map: AHashMap<String, SearchEntry>,
    index: SearchIndex<String>,
}

impl<'a> OmaSearch<'a> {
    pub fn new(cache: &'a Cache) -> OmaSearchResult<Self> {
        let sort = PackageSort::default().include_virtual();
        let packages = cache.packages(&sort);

        let mut pkg_map = AHashMap::new();

        for pkg in packages {
            if pkg.fullname(true).contains("-dbg") {
                continue;
            }

            let status = if pkg.is_upgradable() {
                PackageStatus::Upgrade
            } else if pkg.is_installed() {
                PackageStatus::Installed
            } else {
                PackageStatus::Avail
            };

            if let Some(cand) = pkg.candidate() {
                if let Entry::Vacant(e) = pkg_map.entry(pkg.fullname(true)) {
                    e.insert(SearchEntry {
                        name: pkg.fullname(true),
                        description: format_description(
                            &cand.description().unwrap_or("".to_string()),
                        )
                        .0
                        .to_string(),
                        status,
                        provides: pkg.provides().map(|x| x.to_string()).collect(),
                        has_dbg: has_dbg(cache, &pkg, &cand),
                        raw_pkg: unsafe { pkg.unique() }
                            .make_safe()
                            .ok_or(OmaSearchError::PtrIsNone(PtrIsNone))?,
                        section_is_base: cand.section().map(|x| x == "Bases").unwrap_or(false),
                    });
                }
            } else {
                // Provides
                let mut real_pkgs = vec![];
                for i in pkg.provides() {
                    real_pkgs.push((
                        i.name().to_string(),
                        unsafe { i.target_pkg() }
                            .make_safe()
                            .ok_or(OmaSearchError::PtrIsNone(PtrIsNone))?,
                    ));
                }

                for (provide, i) in real_pkgs {
                    let pkg = Package::new(cache, i);

                    let status = if pkg.is_upgradable() {
                        PackageStatus::Upgrade
                    } else if pkg.is_installed() {
                        PackageStatus::Installed
                    } else {
                        PackageStatus::Avail
                    };

                    if let Some(cand) = pkg.candidate() {
                        pkg_map
                            .entry(pkg.fullname(true))
                            .and_modify(|x| {
                                if !x.provides.contains(&provide) {
                                    x.provides.push(provide.clone())
                                }
                            })
                            .or_insert(SearchEntry {
                                name: pkg.fullname(true),
                                description: format_description(
                                    &cand.description().unwrap_or("".to_string()),
                                )
                                .0
                                .to_string(),
                                status,
                                provides: vec![provide.clone()],
                                has_dbg: has_dbg(cache, &pkg, &cand),
                                raw_pkg: unsafe { pkg.unique() }
                                    .make_safe()
                                    .ok_or(OmaSearchError::PtrIsNone(PtrIsNone))?,
                                section_is_base: cand
                                    .section()
                                    .map(|x| x == "Bases")
                                    .unwrap_or(false),
                            });
                    }
                }
            }
        }

        let mut search_index: SearchIndex<String> = SearchIndex::default();

        pkg_map
            .iter()
            .for_each(|(key, value)| search_index.insert(key, value));

        Ok(Self {
            cache,
            pkg_map,
            index: search_index,
        })
    }

    pub fn search(&self, query: &str) -> OmaSearchResult<Vec<SearchResult>> {
        let mut search_res = vec![];
        let query = query.to_lowercase();
        let res = self.index.search(&query);

        if res.is_empty() {
            return Err(OmaSearchError::NoResult(query));
        }

        for i in res {
            let entry = self.search_result(i, Some(&query))?;
            search_res.push(entry);
        }

        search_res.sort_by(|a, b| b.status.cmp(&a.status));

        for i in 0..search_res.len() {
            if search_res[i].full_match {
                let i = search_res.remove(i);
                search_res.insert(0, i);
            }
        }

        Ok(search_res)
    }

    pub fn search_result(
        &self,
        i: &str,
        query: Option<&str>,
    ) -> Result<SearchResult, OmaSearchError> {
        let entry = self.pkg_map.get(i).unwrap();
        let search_name = entry.name.clone();
        let desc = entry.description.clone();
        let status = entry.status;
        let has_dbg = entry.has_dbg;
        let pkg = unsafe { entry.raw_pkg.unique() }
            .make_safe()
            .ok_or(OmaSearchError::PtrIsNone(PtrIsNone))?;
        let pkg = Package::new(self.cache, pkg);

        let full_match = if let Some(query) = query {
            query == search_name || entry.provides.iter().any(|x| x == query)
        } else {
            false
        };

        let old_version = if status != PackageStatus::Upgrade {
            None
        } else {
            pkg.installed().map(|x| x.version().to_string())
        };

        let new_version = pkg
            .candidate()
            .map(|x| x.version().to_string())
            .ok_or_else(|| OmaSearchError::FailedGetCandidate(pkg.name().to_string()))?;

        let is_base = entry.section_is_base;

        Ok(SearchResult {
            name: pkg.fullname(true),
            desc,
            old_version,
            new_version,
            full_match,
            dbg_package: has_dbg,
            status,
            is_base,
        })
    }
}

#[test]
fn test() {
    use oma_apt::new_cache;

    let packages = std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("test_file")
        .join("Packages");
    let cache = new_cache!(&[packages.to_string_lossy().to_string()]).unwrap();

    let searcher = OmaSearch::new(&cache).unwrap();
    let res = searcher.search("windows-nt-kernel").unwrap();
    let res2 = searcher.search("pwp").unwrap();

    for i in [res, res2] {
        assert!(i.iter().any(|x| x.name == "qaq"));
        assert!(i.iter().any(|x| x.new_version == "9999:1"));
        assert!(i.iter().any(|x| x.full_match));
        assert!(i.iter().filter(|x| x.name == "qaq").count() == 1)
    }

    let res = searcher.search("qwq").unwrap();
    let res2 = searcher.search("qwqdesktop").unwrap();

    for i in [res, res2] {
        assert!(i.iter().any(|x| x.name == "qwq-desktop"));
        assert!(i.iter().any(|x| x.new_version == "9999:114514"));
        assert!(i.iter().any(|x| x.full_match));
        assert!(i.iter().filter(|x| x.name == "qwq-desktop").count() == 1)
    }

    let res = searcher.search("owo").unwrap();
    let res = res.first().unwrap();

    assert_eq!(res.name, "owo".to_string());
    assert_eq!(res.new_version, "9999:2.6.1-2");
    assert!(res.full_match);
}
