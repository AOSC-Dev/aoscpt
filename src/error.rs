use std::error::Error;
use std::fmt::Display;
use std::io::{self, ErrorKind};

use oma_console::error;
use oma_contents::OmaContentsError;
use oma_fetch::checksum::ChecksumError;
use oma_fetch::DownloadError;
use oma_pm::apt::{AptArgsBuilderError, OmaAptArgsBuilderError};
use oma_pm::search::OmaSearchError;
use oma_pm::{apt::OmaAptError, query::OmaDatabaseError};
use oma_refresh::db::RefreshError;
use oma_refresh::inrelease::InReleaseParserError;
use oma_refresh::verify::VerifyError;
use oma_utils::dbus::zError;
use oma_utils::dpkg::DpkgError;

#[cfg(feature = "aosc")]
use oma_topics::OmaTopicsError;

use crate::fl;
use crate::table::print_unmet_dep;

use self::ChainState::*;

use std::vec;

#[derive(Clone)]
pub(crate) enum ChainState<'a> {
    Linked {
        next: Option<&'a (dyn Error + 'static)>,
    },
    Buffered {
        rest: vec::IntoIter<&'a (dyn Error + 'static)>,
    },
}

pub struct Chain<'a> {
    state: ChainState<'a>,
}

impl<'a> Chain<'a> {
    /// Construct an iterator over a chain of errors via the `source` method
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::error::Error;
    /// use std::fmt::{self, Write};
    /// use eyre::Chain;
    /// use indenter::indented;
    ///
    /// fn report(error: &(dyn Error + 'static), f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///     let mut errors = Chain::new(error).enumerate();
    ///     for (i, error) in errors {
    ///         writeln!(f)?;
    ///         write!(indented(f).ind(i), "{}", error)?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(head: &'a (dyn Error + 'static)) -> Self {
        Chain {
            state: ChainState::Linked { next: Some(head) },
        }
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            Linked { next } => {
                let error = (*next)?;
                *next = error.source();
                Some(error)
            }
            Buffered { rest } => rest.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for Chain<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            Linked { mut next } => {
                let mut rest = Vec::new();
                while let Some(cause) = next {
                    next = cause.source();
                    rest.push(cause);
                }
                let mut rest = rest.into_iter();
                let last = rest.next_back();
                self.state = Buffered { rest };
                last
            }
            Buffered { rest } => rest.next_back(),
        }
    }
}

impl ExactSizeIterator for Chain<'_> {
    fn len(&self) -> usize {
        match &self.state {
            Linked { mut next } => {
                let mut len = 0;
                while let Some(cause) = next {
                    next = cause.source();
                    len += 1;
                }
                len
            }
            Buffered { rest } => rest.len(),
        }
    }
}

impl Default for Chain<'_> {
    fn default() -> Self {
        Chain {
            state: ChainState::Buffered {
                rest: Vec::new().into_iter(),
            },
        }
    }
}

#[derive(Debug)]
pub struct OutputError {
    pub description: String,
    pub source: Option<Box<dyn Error>>,
}

impl Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description)
    }
}

impl Error for OutputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<OmaAptError> for OutputError {
    fn from(value: OmaAptError) -> Self {
        oma_apt_error_to_output(value)
    }
}

impl From<OmaAptArgsBuilderError> for OutputError {
    fn from(value: OmaAptArgsBuilderError) -> Self {
        Self {
            description: value.to_string(),
            source: None,
        }
    }
}

impl From<zError> for OutputError {
    fn from(value: zError) -> Self {
        Self {
            description: value.to_string(),
            source: None,
        }
    }
}

impl From<AptArgsBuilderError> for OutputError {
    fn from(value: AptArgsBuilderError) -> Self {
        Self {
            description: value.to_string(),
            source: None,
        }
    }
}

impl From<OmaDatabaseError> for OutputError {
    fn from(value: OmaDatabaseError) -> Self {
        oma_database_error(value)
    }
}

impl From<RefreshError> for OutputError {
    fn from(value: RefreshError) -> Self {
        match value {
            RefreshError::InvaildUrl(_) => Self {
                description: fl!("invaild-url"),
                source: None,
            },
            RefreshError::ParseDistroRepoDataError(path, e) => Self {
                description: fl!("can-not-parse-sources-list", path = path),
                source: Some(Box::new(e)),
            },
            RefreshError::ScanSourceError(e) => Self {
                description: e.to_string(),
                source: None,
            },
            RefreshError::UnsupportedProtocol(s) => Self {
                description: fl!("unsupport-protocol", url = s),
                source: None,
            },
            RefreshError::FetcherError(e) => oma_download_error(e),
            RefreshError::ReqwestError(e) => OutputError::from(e),
            #[cfg(feature = "aosc")]
            RefreshError::TopicsError(e) => oma_topics_error(e),
            #[cfg(not(feature = "aosc"))]
            RefreshError::TopicsError(_) => unreachable!(),
            RefreshError::NoInReleaseFile(s) => Self {
                description: fl!("not-found", url = s),
                source: None,
            },
            RefreshError::InReleaseParserError(e) => match e {
                InReleaseParserError::VerifyError(e) => match e {
                    VerifyError::CertParseFileError(p, e) => Self {
                        description: fl!("fail-load-certs-from-file", path = p),
                        source: Some(Box::new(io::Error::new(ErrorKind::Other, e))),
                    },
                    VerifyError::BadCertFile(p, e) => Self {
                        description: fl!("cert-file-is-bad", path = p),
                        source: Some(Box::new(io::Error::new(ErrorKind::Other, e))),
                    },
                    VerifyError::TrustedDirNotExist => Self {
                        description: e.to_string(),
                        source: None,
                    },
                    VerifyError::Anyhow(e) => Self {
                        description: e.to_string(),
                        source: None,
                    },
                    VerifyError::FailedToReadInRelease(e) => Self {
                        description: "Failed to read decoded InRelease file.".to_string(),
                        source: Some(Box::new(e)),
                    },
                },
                InReleaseParserError::BadInReleaseData => Self {
                    description: fl!("can-not-parse-date"),
                    source: None,
                },
                InReleaseParserError::BadInReleaseVaildUntil => Self {
                    description: fl!("can-not-parse-valid-until"),
                    source: None,
                },
                InReleaseParserError::EarlierSignature(p) => Self {
                    description: fl!("earlier-signature", filename = p),
                    source: None,
                },
                InReleaseParserError::ExpiredSignature(p) => Self {
                    description: fl!("expired-signature", filename = p),
                    source: None,
                },
                InReleaseParserError::BadSha256Value(_) => Self {
                    description: fl!("inrelease-sha256-empty"),
                    source: None,
                },
                InReleaseParserError::BadChecksumEntry(line) => Self {
                    description: fl!("inrelease-checksum-can-not-parse", i = line),
                    source: None,
                },
                InReleaseParserError::InReleaseSyntaxError(p, e) => Self {
                    description: fl!("inrelease-syntax-error", path = p),
                    source: Some(Box::new(io::Error::new(ErrorKind::Other, e))),
                },
                InReleaseParserError::UnsupportFileType => Self {
                    description: fl!("inrelease-parse-unsupport-file-type"),
                    source: None,
                },
                InReleaseParserError::ParseIntError(e) => Self {
                    description: e.to_string(),
                    source: None,
                },
            },
            RefreshError::DpkgArchError(e) => OutputError::from(e),
            RefreshError::JoinError(e) => Self {
                description: e.to_string(),
                source: None,
            },
            RefreshError::DownloadEntryBuilderError(e) => Self {
                description: e.to_string(),
                source: None,
            },
            RefreshError::ChecksumError(e) => oma_checksum_error(e),
            RefreshError::FailedToOperateDirOrFile(path, e) => Self {
                description: format!("Failed to operate path: {path}"),
                source: Some(Box::new(e)),
            },
        }
    }
}

#[cfg(feature = "aosc")]
impl From<OmaTopicsError> for OutputError {
    fn from(value: OmaTopicsError) -> Self {
        oma_topics_error(value)
    }
}

#[cfg(feature = "aosc")]
fn oma_topics_error(e: OmaTopicsError) -> OutputError {
    match e {
        OmaTopicsError::BrokenFile(_) => OutputError {
            description: fl!("failed-to-read"),
            source: None,
        },
        OmaTopicsError::FailedToOperateDirOrFile(path, e) => OutputError {
            description: format!("Failed to operate path: {path}"),
            source: Some(Box::new(e)),
        },
        OmaTopicsError::CanNotFindTopic(topic) => OutputError {
            description: fl!("can-not-find-specified-topic", topic = topic),
            source: None,
        },
        OmaTopicsError::FailedToDisableTopic(topic) => OutputError {
            description: fl!("can-not-find-specified-topic", topic = topic),
            source: None,
        },
        OmaTopicsError::ReqwestError(e) => OutputError::from(e),
        OmaTopicsError::FailedSer => OutputError {
            description: e.to_string(),
            source: None,
        },
    }
}

impl From<DpkgError> for OutputError {
    fn from(value: DpkgError) -> Self {
        Self {
            description: fl!("can-not-run-dpkg-print-arch"),
            source: Some(Box::new(value)),
        }
    }
}

impl From<DownloadError> for OutputError {
    fn from(value: DownloadError) -> Self {
        oma_download_error(value)
    }
}

impl From<OmaContentsError> for OutputError {
    fn from(value: OmaContentsError) -> Self {
        #[cfg(feature = "contents-without-rg")]
        let s = match value {
            OmaContentsError::ContentsNotExist => Self {
                description: fl!("contents-does-not-exist"),
                source: None,
            },
            OmaContentsError::ContentsEntryMissingPathList(s) => Self {
                description: fl!("contents-entry-missing-path-list", entry = s),
                source: None,
            },
            OmaContentsError::CnfWrongArgument => Self {
                description: value.to_string(),
                source: None,
            },
            OmaContentsError::NoResult => Self {
                description: "".to_string(),
                source: None,
            },
            OmaContentsError::LzzzErr(e) => Self {
                description: e.to_string(),
                source: None,
            },
            OmaContentsError::FailedToOperateDirOrFile(path, e) => Self {
                description: format!("Failed to operate dir or file: {path}"),
                source: Some(Box::new(e)),
            },
            OmaContentsError::FailedToGetFileMetadata(path, e) => Self {
                description: format!("Failed to get file metadata: {path}"),
                source: Some(Box::new(e)),
            },
        };

        #[cfg(not(feature = "contents-without-rg"))]
        let s = match value {
            OmaContentsError::ContentsNotExist => Self {
                description: fl!("contents-does-not-exist"),
                source: None,
            },
            OmaContentsError::ExecuteRgFailed(e) => Self {
                description: fl!("execute-ripgrep-failed"),
                source: Some(Box::new(e)),
            },
            OmaContentsError::ContentsEntryMissingPathList(s) => Self {
                description: fl!("contents-entry-missing-path-list", entry = s),
                source: None,
            },
            OmaContentsError::CnfWrongArgument => Self {
                description: value.to_string(),
                source: None,
            },
            OmaContentsError::RgWithError => Self {
                description: fl!("rg-non-zero"),
                source: None,
            },
            OmaContentsError::NoResult => Self {
                description: "".to_string(),
                source: None,
            },
            OmaContentsError::FailedToOperateDirOrFile(path, e) => Self {
                description: format!("Failed to operate dir or file: {path}"),
                source: Some(Box::new(e)),
            },
            OmaContentsError::FailedToGetFileMetadata(path, e) => Self {
                description: format!("Failed to get file metadata: {path}"),
                source: Some(Box::new(e)),
            },
            OmaContentsError::FailedToWaitExit(e) => Self {
                description: "Failed to wait `rg' to exit.".to_string(),
                source: Some(Box::new(e)),
            },
        };

        s
    }
}

impl From<anyhow::Error> for OutputError {
    fn from(value: anyhow::Error) -> Self {
        Self {
            description: value.to_string(),
            source: None,
        }
    }
}

pub fn oma_apt_error_to_output(err: OmaAptError) -> OutputError {
    match err {
        OmaAptError::RustApt(e) => OutputError {
            description: fl!("apt-error"),
            source: Some(Box::new(e)),
        },
        OmaAptError::OmaDatabaseError(e) => oma_database_error(e),
        OmaAptError::MarkReinstallError(pkg, version) => OutputError {
            description: fl!("can-not-mark-reinstall", name = pkg, version = version),
            source: None,
        },
        OmaAptError::DependencyIssue(ref v) => match v {
            v if v.is_empty() || print_unmet_dep(v).is_err() => OutputError {
                description: err.to_string(),
                source: None,
            },
            _ => OutputError {
                description: "".to_string(),
                source: None,
            },
        },
        OmaAptError::PkgIsEssential(s) => OutputError {
            description: fl!("pkg-is-essential", name = s),
            source: None,
        },
        OmaAptError::PkgNoCandidate(s) => OutputError {
            description: fl!("no-candidate-ver", pkg = s),
            source: None,
        },
        OmaAptError::PkgNoChecksum(s) => OutputError {
            description: fl!("pkg-no-checksum", name = s),
            source: None,
        },
        OmaAptError::InvalidFileName(s) => OutputError {
            description: fl!("invaild-filename", name = s),
            source: None,
        },
        OmaAptError::DownlaodError(e) => oma_download_error(e),
        OmaAptError::InstallEntryBuilderError(e) => OutputError {
            description: e.to_string(),
            source: None,
        },
        OmaAptError::DpkgFailedConfigure(e) => OutputError {
            description: fl!("dpkg-configure-a-non-zero"),
            source: Some(Box::new(e)),
        },
        OmaAptError::DiskSpaceInsufficient(need, avail) => OutputError {
            description: fl!(
                "need-more-size",
                a = avail.to_string(),
                n = need.to_string()
            ),
            source: None,
        },
        OmaAptError::DownloadEntryBuilderError(e) => OutputError {
            description: e.to_string(),
            source: None,
        },
        OmaAptError::CommitErr(e) => OutputError {
            description: e,
            source: None,
        },
        OmaAptError::MarkPkgNotInstalled(pkg) => OutputError {
            description: fl!("pkg-is-not-installed", pkg = pkg),
            source: None,
        },
        OmaAptError::DpkgError(e) => OutputError::from(e),
        OmaAptError::PkgUnavailable(pkg, ver) => OutputError {
            description: fl!("pkg-unavailable", pkg = pkg, ver = ver),
            source: None,
        },
        OmaAptError::FailedToDownload(size, errs) => {
            for i in errs {
                let err = oma_download_error(i);
                error!("{}", err.description);
            }
            OutputError {
                description: fl!("download-failed-with-len", len = size),
                source: None,
            }
        }
        OmaAptError::FailedCreateAsyncRuntime(e) => OutputError {
            description: "Failed to create async runtime".to_string(),
            source: Some(Box::new(e)),
        },
        OmaAptError::FailedOperateDirOrFile(path, e) => OutputError {
            description: format!("Failed to operate dir or file: {path}"),
            source: Some(Box::new(e)),
        },
        OmaAptError::FailedGetAvailableSpace(e) => OutputError {
            description: "Failed to get available space".to_string(),
            source: Some(Box::new(e)),
        },
    }
}

impl From<reqwest::Error> for OutputError {
    fn from(e: reqwest::Error) -> Self {
        let filename = &e
            .url()
            .and_then(|x| x.path_segments())
            .and_then(|x| x.last());

        if e.is_builder() {
            return Self {
                description: "Failed to create http client.".to_string(),
                source: Some(Box::new(e)),
            };
        }

        if let Some(filename) = filename {
            Self {
                description: fl!("download-failed", filename = filename.to_string()),
                source: Some(Box::new(e)),
            }
        } else {
            Self {
                description: fl!("download-failed"),
                source: None,
            }
        }
    }
}

fn oma_download_error(e: DownloadError) -> OutputError {
    match e {
        DownloadError::ChecksumMisMatch(filename) => OutputError {
            description: fl!("checksum-mismatch", filename = filename),
            source: None,
        },
        DownloadError::IOError(s, e) => OutputError {
            description: fl!("download-failed", filename = s),
            source: Some(Box::new(e)),
        },
        DownloadError::ReqwestError(e) => OutputError::from(e),
        DownloadError::ChecksumError(e) => oma_checksum_error(e),
        DownloadError::FailedOpenLocalSourceFile(path, e) => OutputError {
            description: fl!("can-not-parse-sources-list", path = path.to_string()),
            source: Some(Box::new(e)),
        },
        DownloadError::DownloadSourceBuilderError(e) => OutputError {
            description: e.to_string(),
            source: None,
        },
        DownloadError::InvaildURL(s) => OutputError {
            description: fl!("invaild-url", url = s),
            source: None,
        },
    }
}

fn oma_checksum_error(e: ChecksumError) -> OutputError {
    match e {
        ChecksumError::FailedToOpenFile(s, e) => OutputError {
            description: fl!("failed-to-open-to-checksum", path = s),
            source: Some(Box::new(e)),
        },
        ChecksumError::ChecksumIOError(e) => OutputError {
            description: fl!("can-not-checksum"),
            source: Some(Box::new(e)),
        },
        ChecksumError::BadLength => OutputError {
            description: fl!("sha256-bad-length"),
            source: None,
        },
        ChecksumError::HexError(e) => OutputError {
            description: e.to_string(),
            source: None,
        },
    }
}

fn oma_database_error(e: OmaDatabaseError) -> OutputError {
    match e {
        OmaDatabaseError::RustApt(e) => OutputError {
            description: fl!("apt-error"),
            source: Some(Box::new(e)),
        },
        OmaDatabaseError::InvaildPattern(s) => OutputError {
            description: fl!("invaild-pattern", p = s),
            source: None,
        },
        OmaDatabaseError::NoPackage(s) => OutputError {
            description: fl!("can-not-get-pkg-from-database", name = s),
            source: None,
        },
        OmaDatabaseError::NoVersion(pkg, ver) => OutputError {
            description: fl!("pkg-unavailable", pkg = pkg, ver = ver),
            source: None,
        },
        OmaDatabaseError::NoPath(s) => OutputError {
            description: fl!("invaild-path", p = s),
            source: None,
        },
        OmaDatabaseError::OmaSearchError(e) => match e {
            OmaSearchError::RustApt(e) => OutputError {
                description: fl!("apt-error"),
                source: Some(Box::new(e)),
            },
            OmaSearchError::NoResult(e) => OutputError {
                description: fl!("could-not-find-pkg-from-keyword", c = e),
                source: None,
            },
            OmaSearchError::FailedGetCandidate(s) => OutputError {
                description: fl!("no-candidate-ver", pkg = s),
                source: None,
            },
        },
        OmaDatabaseError::NoCandidate(s) => OutputError {
            description: fl!("no-candidate-ver", pkg = s),
            source: None,
        },
    }
}
