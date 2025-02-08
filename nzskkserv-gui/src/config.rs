use std::path::PathBuf;

use directories::ProjectDirs;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::fs;

use anyhow::Context;
use anyhow::Result;
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

impl From<Encoding> for nzskkserv_core::Encoding {
    fn from(value: Encoding) -> Self {
        match value {
            Encoding::Utf8 => nzskkserv_core::Encoding::Utf8,
            Encoding::Eucjp => nzskkserv_core::Encoding::Eucjp,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DictPath {
    File(PathBuf),
    Url(Url),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DictDef {
    pub path: DictPath,
    pub encoding: Option<Encoding>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub(crate) struct Config {
    pub enable_google_cgi: bool,
    pub server_encoding: Encoding,
    pub port: Option<u16>,
    pub dicts: Vec<DictDef>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enable_google_cgi: false,
            server_encoding: Encoding::Utf8,
            dicts: Vec::new(),
            port: Some(1178),
        }
    }
}

pub(crate) async fn load_config() -> Result<Config> {
    let config = match read_config().await {
        Ok(config) => config,
        Err(_e) => {
            info!("Could not read config. Creating new one...");
            write_config(&Config::default()).await?;
            Config::default()
        }
    };
    Ok(config)
}

async fn read_config() -> Result<Config> {
    let project_dirs = ProjectDirs::from("", "", "nzskkserv").context("No project dirs")?;
    let mut config_path = project_dirs.config_dir().to_path_buf();
    fs::create_dir_all(&config_path).await?;
    config_path.push("config.toml");

    let config_file = tokio::fs::read_to_string(&config_path).await?;
    let config: Config = toml::from_str(&config_file)?;
    Ok(config)
}

pub(crate) async fn write_config(config: &Config) -> Result<()> {
    let project_dirs =
        ProjectDirs::from("", "", "nzskkserv").context("Could not find config dir.")?;
    let mut config_file_path = project_dirs.config_dir().to_path_buf();

    fs::create_dir_all(&config_file_path).await?;

    config_file_path.push("config.toml");

    let config_text = toml::to_string(&config)?;
    fs::write(config_file_path, config_text).await?;

    Ok(())
}
