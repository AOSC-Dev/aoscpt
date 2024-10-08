use std::{
    borrow::Cow,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};

use ahash::RandomState;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt};
use tracing::{debug, warn};
use url::Url;

pub type Result<T> = std::result::Result<T, OmaTopicsError>;
type IndexMap<K, V> = indexmap::IndexMap<K, V, RandomState>;

#[derive(Debug, thiserror::Error)]
pub enum OmaTopicsError {
    #[error("Failed to operate dir or flle {0}: {1}")]
    FailedToOperateDirOrFile(String, std::io::Error),
    #[error("Can not find topic: {0}")]
    CanNotFindTopic(String),
    #[error("Failed to enable topic: {0}")]
    FailedToDisableTopic(String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to serialize data")]
    FailedSer,
    #[error("Failed to get path parent: {0:?}")]
    FailedGetParentPath(PathBuf),
    #[error("file is broken")]
    BrokenFile(String),
    #[error("Failed to Parse Url: {0}")]
    ParseUrl(url::ParseError),
    #[error("Unsupported url protocol from url: {0}")]
    UnsupportedProtocol(String),
    #[error("Failed to open file {0}: {1}")]
    OpenFile(String, io::Error),
    #[error("Failed to read file {0}: {1}")]
    ReadFile(String, serde_json::Error),
}

#[derive(Deserialize)]
struct GenList {
    mirror: IndexMap<String, String>,
}

async fn enabled_mirror<P: AsRef<Path>>(rootfs: P) -> Result<Vec<String>> {
    let apt_gen_list = rootfs.as_ref().join("var/lib/apt/gen/status.json");
    let s = tokio::fs::read_to_string(&apt_gen_list)
        .await
        .map_err(|e| {
            OmaTopicsError::FailedToOperateDirOrFile(apt_gen_list.display().to_string(), e)
        })?;

    let gen_list: GenList = serde_json::from_str(&s).map_err(|_| {
        OmaTopicsError::BrokenFile(
            apt_gen_list
                .file_name()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .to_string(),
        )
    })?;

    let urls = gen_list
        .mirror
        .values()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    Ok(urls)
}

const TOPICS_JSON: &str = "manifest/topics.json";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Topic {
    pub name: String,
    pub description: Option<String>,
    date: u64,
    update_date: Option<u64>,
    #[serde(skip_serializing)]
    arch: Option<Vec<String>>,
    pub packages: Vec<String>,
}

#[derive(Debug)]
pub struct TopicManager<'a> {
    enabled: IndexMap<String, Topic>,
    all: IndexMap<String, Topic>,
    client: &'a Client,
    atm_state_path: PathBuf,
    arch: &'a str,
    sysroot: &'a Path,
}

impl<'a> TopicManager<'a> {
    const ATM_STATE_PATH_SUFFIX: &'a str = "var/lib/atm/state";
    const ATM_STATE_SOURCE_LIST_PATH_SUFFIX: &'a str = "etc/apt/sources.list.d/atm.list";

    pub async fn new(client: &'a Client, sysroot: &'a Path, arch: &'a str) -> Result<Self> {
        let atm_state_path = sysroot.join(Self::ATM_STATE_PATH_SUFFIX);

        if !atm_state_path.exists() {
            let parent = atm_state_path
                .parent()
                .ok_or_else(|| OmaTopicsError::FailedGetParentPath(atm_state_path.to_path_buf()))?;

            if !parent.is_dir() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                    OmaTopicsError::FailedToOperateDirOrFile(parent.display().to_string(), e)
                })?;
            }

            if !atm_state_path.is_file() {
                tokio::fs::File::create(&atm_state_path)
                    .await
                    .map_err(|e| {
                        OmaTopicsError::FailedToOperateDirOrFile(
                            atm_state_path.display().to_string(),
                            e,
                        )
                    })?;
            }
        }

        let atm_state_string = tokio::fs::read_to_string(&atm_state_path)
            .await
            .map_err(|e| {
                OmaTopicsError::FailedToOperateDirOrFile(atm_state_path.display().to_string(), e)
            })?;

        Ok(Self {
            enabled: serde_json::from_str(&atm_state_string).unwrap_or_else(|e| {
                debug!("Deserialize oma topics state JSON failed: {e}");
                warn!("oma topics status file does not exist or is corrupted, a new status file will be created.");
                IndexMap::with_hasher(ahash::RandomState::new())
            }),
            all: IndexMap::with_hasher(ahash::RandomState::new()),
            client,
            atm_state_path,
            arch,
            sysroot,
        })
    }

    pub fn all_topics(&self) -> &IndexMap<String, Topic> {
        &self.all
    }

    pub fn enabled_topics(&self) -> &IndexMap<String, Topic> {
        &self.enabled
    }

    /// Get all new topics
    pub async fn refresh(&mut self) -> Result<()> {
        let urls = enabled_mirror(self.sysroot)
            .await?
            .iter()
            .map(|x| {
                if x.ends_with('/') {
                    format!("{}debs/{TOPICS_JSON}", x)
                } else {
                    format!("{}/debs/{TOPICS_JSON}", x)
                }
            })
            .collect::<Vec<_>>();

        self.all = refresh_innter(&self.client, urls, self.arch).await?;

        Ok(())
    }

    /// Enable select topic
    pub fn add(&mut self, topic: &str, dry_run: bool) -> Result<()> {
        debug!("oma will opt_in: {}", topic);

        if dry_run {
            return Ok(());
        }

        if let Some(index) = self.all.get(topic) {
            if !self.enabled.get(topic).is_none() {
                self.enabled.insert(topic.to_string(), index.clone());
            }

            return Ok(());
        }

        debug!("topic does not exist");

        Err(OmaTopicsError::CanNotFindTopic(topic.to_owned()))
    }

    /// Disable select topic
    pub fn remove(&mut self, topic: &str, dry_run: bool) -> Result<Topic> {
        debug!("oma will opt_out: {}", topic);
        debug!("topic is: {topic}");
        debug!("enabled topics: {:?}", self.enabled);

        if dry_run {
            let index = self.enabled.get(topic);

            debug!("index is: {index:?}");

            return Ok(index.unwrap().to_owned());
        }

        self.enabled
            .shift_remove(topic)
            .ok_or_else(|| OmaTopicsError::FailedToDisableTopic(topic.to_string()))
    }

    /// Write topic changes to mirror list
    pub async fn write_enabled(
        &self,
        dry_run: bool,
        comment: &str,
        message_cb: impl Fn(&str, &str),
    ) -> Result<()> {
        if dry_run {
            return Ok(());
        }

        let source_list_path = self.sysroot.join(Self::ATM_STATE_SOURCE_LIST_PATH_SUFFIX);

        let mut f = tokio::fs::File::create(&source_list_path)
            .await
            .map_err(|e| {
                OmaTopicsError::FailedToOperateDirOrFile(source_list_path.display().to_string(), e)
            })?;

        let mirrors = enabled_mirror(self.atm_state_path.as_path()).await?;

        f.write_all(format!("{}\n", comment).as_bytes())
            .await
            .map_err(|e| {
                OmaTopicsError::FailedToOperateDirOrFile(source_list_path.display().to_string(), e)
            })?;

        for i in &self.enabled {
            f.write_all(format!("# Topic `{}`\n", i.0).as_bytes())
                .await
                .map_err(|e| {
                    OmaTopicsError::FailedToOperateDirOrFile(
                        source_list_path.display().to_string(),
                        e,
                    )
                })?;

            let mut tasks = vec![];
            for mirror in &mirrors {
                tasks.push(self.mirror_topic_is_exist(format!(
                    "{}debs/dists/{}",
                    url_with_suffix(mirror),
                    i.0
                )))
            }

            let is_exists = futures::future::join_all(tasks).await;

            for (index, c) in is_exists.into_iter().enumerate() {
                if !c.unwrap_or(false) {
                    message_cb(&i.0, &mirrors[index]);
                    continue;
                }

                f.write_all(
                    format!(
                        "deb {}debs {} main\n",
                        url_with_suffix(&mirrors[index]),
                        i.0
                    )
                    .as_bytes(),
                )
                .await
                .map_err(|e| {
                    OmaTopicsError::FailedToOperateDirOrFile(
                        source_list_path.display().to_string(),
                        e,
                    )
                })?;
            }
        }

        let s = serde_json::to_vec(&self.enabled).map_err(|_| OmaTopicsError::FailedSer)?;

        tokio::fs::write(&self.atm_state_path, s)
            .await
            .map_err(|e| {
                OmaTopicsError::FailedToOperateDirOrFile(
                    self.atm_state_path.display().to_string(),
                    e,
                )
            })?;

        Ok(())
    }

    async fn mirror_topic_is_exist(&self, url: String) -> Result<bool> {
        check(&self.client, &format!("{}/InRelease", url)).await
    }
}

async fn get<T: DeserializeOwned>(client: &Client, url: String) -> Result<T> {
    let url = Url::parse(&url).map_err(OmaTopicsError::ParseUrl)?;

    let schema = url.scheme();

    match schema {
        "file" => {
            let path = url.path();
            let bytes = fs::read(path)
                .await
                .map_err(|e| OmaTopicsError::OpenFile(path.to_string(), e))?;
            serde_json::from_slice(&bytes)
                .map_err(|e| OmaTopicsError::ReadFile(path.to_string(), e))
        }
        x if x.starts_with("http") => {
            let res = client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json::<T>()
                .await?;
            Ok(res)
        }
        _ => Err(OmaTopicsError::UnsupportedProtocol(url.to_string())),
    }
}

async fn check(client: &Client, url: &str) -> Result<bool> {
    let url = Url::parse(url).map_err(OmaTopicsError::ParseUrl)?;

    let schema = url.scheme();

    match schema {
        "file" => Ok(Path::new(url.path()).exists()),
        x if x.starts_with("http") => Ok(client.head(url).send().await?.error_for_status().is_ok()),
        _ => Err(OmaTopicsError::UnsupportedProtocol(url.to_string())),
    }
}

fn url_with_suffix(url: &str) -> Cow<str> {
    if url.ends_with('/') {
        Cow::Borrowed(url)
    } else {
        Cow::Owned(format!("{url}/"))
    }
}

async fn refresh_innter(
    client: &Client,
    urls: Vec<String>,
    arch: &str,
) -> Result<IndexMap<String, Topic>> {
    let mut json: Vec<Topic> = vec![];
    let mut tasks = vec![];

    for url in urls {
        let v = get::<Vec<Topic>>(client, url);
        tasks.push(v);
    }

    let res = futures::future::try_join_all(tasks).await?;

    for i in res {
        for j in i {
            match json.iter().position(|x| x.name == j.name) {
                Some(index) => {
                    if j.update_date > json[index].update_date {
                        json[index] = j.clone();
                    }
                }
                None => {
                    json.push(j);
                }
            }
        }
    }

    json.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    let json = json
        .into_iter()
        .filter(|x| {
            x.arch
                .as_ref()
                .map(|x| x.iter().any(|x| x == arch || x == "all"))
                .unwrap_or(false)
        })
        .map(|x| (x.name.clone(), x))
        .collect::<IndexMap<_, _>>();

    Ok(json)
}

/// Scan all close topics from upstream and disable it
pub async fn scan_closed_topic(
    comment: &str,
    message_cb: impl Fn(&str, &str),
    tm: &mut TopicManager<'_>,
) -> Result<Vec<String>> {
    tm.refresh().await?;
    let all = tm.all_topics().to_owned();

    let enabled = tm.enabled_topics().to_owned();

    let mut res = vec![];

    for i in enabled {
        if all.get(&i.0).is_none() {
            let d = tm.remove(&i.0, false)?;
            res.push(d.name);
        }
    }

    tm.write_enabled(false, comment, message_cb).await?;

    Ok(res)
}
