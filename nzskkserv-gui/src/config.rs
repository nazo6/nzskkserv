use std::path::PathBuf;

use directories::ProjectDirs;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::fs;

use anyhow::Context;
use anyhow::Result;
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DictDef {
    File {
        path: PathBuf,
        encoding: Option<Encoding>,
    },
    Url {
        url: Url,
        encoding: Option<Encoding>,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

pub(super) async fn load_config() -> Result<Config> {
    let project_dirs = ProjectDirs::from("", "", "nzskkserv").context("No project dirs")?;
    let mut config_path = project_dirs.config_dir().to_path_buf();
    config_path.push("config.toml");

    info!("Reading config from {:?}", config_path);

    let config = if fs::metadata(&config_path).await.is_ok() {
        let config_file = tokio::fs::read_to_string(&config_path).await?;
        toml::from_str(&config_file)?
    } else {
        info!("Config file not found. Creating new one...");
        write_config(&Config::default()).await?;
        Config::default()
    };

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
