use crate::fl;
use anyhow::Result;
use oma_console::warn;
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &str = include_str!("../data/config/oma.toml");

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub network: NetworkConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub protect_essentials: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub network_threads: usize,
}

impl Config {
    pub fn read() -> Result<Self> {
        let s = std::fs::read_to_string("/etc/oma.toml");

        Ok(match s {
            Ok(s) => toml::from_str(&s)?,
            Err(_) => {
                warn!("{}", fl!("config-invaild"));
                toml::from_str(DEFAULT_CONFIG)?
            }
        })
    }
}