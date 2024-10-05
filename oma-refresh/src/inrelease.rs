use ahash::AHashMap;
use chrono::{DateTime, FixedOffset, ParseError, Utc};
use once_cell::sync::OnceCell;
use std::{borrow::Cow, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumItem {
    pub name: String,
    pub size: u64,
    pub checksum: String,
}

#[derive(Debug, thiserror::Error)]
pub enum InReleaseError {
    #[error("Mirror is not signed by trusted keyring.")]
    NotTrusted,
    #[error(transparent)]
    VerifyError(#[from] oma_repo_verify::VerifyError),
    #[error("Bad InRelease Data")]
    BadInReleaseData,
    #[error("Bad valid until")]
    BadInReleaseValidUntil,
    #[error("Earlier signature")]
    EarlierSignature,
    #[error("Expired signature")]
    ExpiredSignature,
    #[error("Bad InRelease")]
    InReleaseSyntaxError,
    #[error("Unsupported file type in path")]
    UnsupportedFileType,
    #[error(transparent)]
    ParseIntError(ParseIntError),
    #[error("InRelease is broken")]
    BrokenInRelease,
}

pub type InReleaseParserResult<T> = Result<T, InReleaseError>;

#[derive(Clone, Copy)]
pub enum InReleaseChecksum {
    Sha256,
    Sha512,
    Md5,
}

const COMPRESS: &[&str] = &[".gz", ".xz", ".zst", ".bz2"];

pub struct InRelease<'a> {
    source: AHashMap<&'a str, String>,
    acquire_by_hash: OnceCell<bool>,
    checksum_type_and_list: OnceCell<(InReleaseChecksum, Vec<ChecksumItem>)>,
}

impl<'a> InRelease<'a> {
    pub fn new(input: &'a str) -> Result<Self, InReleaseError> {
        let mut map = AHashMap::new();
        let debcontrol =
            oma_debcontrol::parse_str(input).map_err(|_| InReleaseError::InReleaseSyntaxError)?;

        let inrelease = debcontrol.first().ok_or(InReleaseError::BrokenInRelease)?;

        for i in &inrelease.fields {
            map.insert(i.name, i.value.to_string());
        }

        Ok(Self {
            source: map,
            acquire_by_hash: OnceCell::new(),
            checksum_type_and_list: OnceCell::new(),
        })
    }

    pub fn try_init(&mut self) -> Result<(), InReleaseError> {
        let sha256 = self.source.get("SHA256");
        let sha512 = self.source.get("SHA512");
        let md5 = self.source.get("MD5Sum");

        self.checksum_type_and_list.get_or_try_init(|| {
            let (checksum_type, checksums) = if let Some(sha256) = sha256 {
                (InReleaseChecksum::Sha256, get_checksums_inner(sha256)?)
            } else if let Some(sha512) = sha512 {
                (InReleaseChecksum::Sha512, get_checksums_inner(sha512)?)
            } else if let Some(md5) = md5 {
                (InReleaseChecksum::Md5, get_checksums_inner(md5)?)
            } else {
                return Err(InReleaseError::BrokenInRelease);
            };

            Ok((checksum_type, checksums))
        })?;

        let _ = *self.acquire_by_hash.get_or_init(|| {
            self.source
                .get("Acquire-By-Hash")
                .map(|x| x.to_lowercase() == "yes")
                .unwrap_or(false)
        });

        Ok(())
    }

    pub fn checksum_type_and_list(&self) -> &(InReleaseChecksum, Vec<ChecksumItem>) {
        self.checksum_type_and_list
            .get()
            .expect("checksum type and list does not init")
    }

    pub fn acquire_by_hash(&self) -> bool {
        *self
            .acquire_by_hash
            .get()
            .expect("acquire_by_hash value does not init")
    }

    pub fn check_date(&self, now: &DateTime<Utc>) -> Result<(), InReleaseError> {
        let date = self
            .source
            .get("Date")
            .ok_or(InReleaseError::BadInReleaseData)?;

        let date = parse_date(date).map_err(|e| {
            debug!("Parse data failed: {}", e);
            InReleaseError::BadInReleaseData
        })?;

        if now < &date {
            return Err(InReleaseError::EarlierSignature);
        }

        Ok(())
    }

    pub fn check_valid_until(&self, now: &DateTime<Utc>) -> Result<(), InReleaseError> {
        // Check if the `Valid-Until` field is valid only when it is defined.
        if let Some(valid_until_date) = self.source.get("Valid-Until") {
            let valid_until = parse_date(valid_until_date).map_err(|e| {
                debug!("Parse valid_until failed: {}", e);
                InReleaseError::BadInReleaseValidUntil
            })?;

            if now > &valid_until {
                return Err(InReleaseError::ExpiredSignature);
            }
        }

        Ok(())
    }
}

impl FromStr for ChecksumItem {
    type Err = InReleaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.splitn(3, |c: char| c.is_ascii_whitespace());
        let checksum = line
            .next()
            .ok_or(InReleaseError::BrokenInRelease)?
            .to_string();
        let size = line
            .next()
            .ok_or(InReleaseError::BrokenInRelease)?
            .parse::<u64>()
            .map_err(InReleaseError::ParseIntError)?;
        let name = line
            .next()
            .ok_or(InReleaseError::BrokenInRelease)?
            .to_string();

        Ok(Self {
            name,
            size,
            checksum,
        })
    }
}

fn get_checksums_inner(checksum_str: &str) -> Result<Vec<ChecksumItem>, InReleaseError> {
    checksum_str
        .trim()
        .lines()
        .map(ChecksumItem::from_str)
        .collect::<Result<Vec<_>, InReleaseError>>()
}

pub fn verify_inrelease<'a>(
    inrelease: &'a str,
    signed_by: Option<&'a str>,
    rootfs: impl AsRef<Path>,
    trusted: bool,
) -> Result<Cow<'a, str>, InReleaseError> {
    if inrelease.starts_with("-----BEGIN PGP SIGNED MESSAGE-----") {
        Ok(Cow::Owned(oma_repo_verify::verify(
            inrelease, signed_by, rootfs,
        )?))
    } else {
        if !trusted {
            return Err(InReleaseError::NotTrusted);
        }
        Ok(Cow::Borrowed(inrelease))
    }
}

pub(crate) fn split_ext_and_filename(x: &str) -> (String, String) {
    let path = Path::new(x);
    let ext = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let name = path.with_extension("");
    let name = name.to_string_lossy().to_string();

    (ext, name)
}

pub(crate) fn file_is_compress(name: &str) -> bool {
    for i in COMPRESS {
        if name.ends_with(i) {
            return true;
        }
    }

    false
}

#[derive(Debug, Error)]
enum ParseDateError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error("Could not parse date: {0}")]
    BadDate(ParseIntError),
}

fn parse_date(date: &str) -> Result<DateTime<FixedOffset>, ParseDateError> {
    match DateTime::parse_from_rfc2822(date) {
        Ok(res) => Ok(res),
        Err(e) => {
            debug!("Parse {} failed: {e}, try to use date hack.", date);
            let hack_date = date_hack(date).map_err(ParseDateError::BadDate)?;
            Ok(DateTime::parse_from_rfc2822(&hack_date)?)
        }
    }
}

/// Replace RFC 1123/822/2822 non-compliant "UTC" marker with RFC 2822-compliant "+0000" whilst parsing InRelease.
/// and for non-standard X:YY:ZZ conversion to XX:YY:ZZ.
///
/// - Some third-party repositories (such as those generated with Aptly) uses "UTC" to denote the Coordinated Universal
///   Time, which is not allowed in RFC 1123 or 822/2822 (all calls for "GMT" or "UT", 822 allows "Z", and 2822 allows
///   "+0000").
/// - This is used by many commercial software vendors, such as Google, Microsoft, and Spotify.
/// - This is allowed in APT's RFC 1123 parser. However, as chrono requires full compliance with the
///   aforementioned RFC documents, "UTC" is considered illegal.
///
/// Replace the "UTC" marker at the end of date strings to make it palatable to chronos.
///
/// and for non-standard X:YY:ZZ conversion to XX:YY:ZZ to make it palatable to chronos.
fn date_hack(date: &str) -> Result<String, ParseIntError> {
    let mut split_time = date
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    for c in split_time.iter_mut() {
        if c.is_empty() || !c.contains(':') {
            continue;
        }

        let mut time_split = c.split(':').map(|x| x.to_string()).collect::<Vec<_>>();

        // X:YY:ZZ conversion to XX:YY:ZZ to make it palatable to chronos
        for k in time_split.iter_mut() {
            match k.parse::<u64>()? {
                0..=9 if k.len() == 1 => {
                    *k = "0".to_string() + k;
                }
                _ => continue,
            }
        }

        *c = time_split.join(":");
    }

    let date = split_time.join(" ");

    Ok(date.replace("UTC", "+0000"))
}

#[test]
fn test_date_hack() {
    let a = "Thu, 02 May 2024  9:58:03 UTC";
    let hack = date_hack(&a).unwrap();
    assert_eq!(hack, "Thu, 02 May 2024 09:58:03 +0000");
    let b = DateTime::parse_from_rfc2822(&hack);
    assert!(b.is_ok());

    let a = "Thu, 02 May 2024 09:58:03 +0000";
    let hack = date_hack(&a).unwrap();
    assert_eq!(hack, "Thu, 02 May 2024 09:58:03 +0000");
    let b = DateTime::parse_from_rfc2822(&hack);
    assert!(b.is_ok());

    let a = "Thu, 02 May 2024  0:58:03 +0000";
    let hack = date_hack(&a).unwrap();
    assert_eq!(hack, "Thu, 02 May 2024 00:58:03 +0000");
    let b = DateTime::parse_from_rfc2822(&hack);
    assert!(b.is_ok());
}

#[test]
fn test_split_name_and_ext() {
    let example1 = "main/dep11/icons-128x128.tar.gz";
    let res = split_ext_and_filename(&example1);
    assert_eq!(
        res,
        ("gz".to_string(), "main/dep11/icons-128x128.tar".to_string())
    );

    let example2 = "main/i18n/Translation-bg.xz";
    let res = split_ext_and_filename(&example2);
    assert_eq!(
        res,
        ("xz".to_string(), "main/i18n/Translation-bg".to_string())
    );

    let example2 = "main/i18n/Translation-bg";
    let res = split_ext_and_filename(&example2);
    assert_eq!(
        res,
        ("".to_string(), "main/i18n/Translation-bg".to_string())
    );
}
