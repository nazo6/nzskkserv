use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::info;

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
    File { path: PathBuf },
    Url { url: Url },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DictDef {
    #[serde(flatten)]
    pub path_or_url: DictPath,
    #[serde(default = "default_encoding")]
    pub encoding: Encoding,
}

fn default_encoding() -> Encoding {
    Encoding::Utf8
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub(crate) struct Config {
    pub enable_google_cgi: bool,
    pub server_encoding: Encoding,
    pub port: u16,
    pub dicts: Vec<DictDef>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enable_google_cgi: false,
            server_encoding: Encoding::Utf8,
            dicts: Vec::new(),
            port: 1178,
        }
    }
}

pub(crate) async fn load_config() -> Result<Config> {
    let project_dirs = ProjectDirs::from("", "", "nzskkserv").context("No project dirs")?;
    let config_dir = project_dirs.config_dir().to_path_buf();
    let config_path = config_dir.join("config.toml");

    let config: Config = if !config_path.exists() {
        info!("Config file not found. Creating default config file.");
        fs::create_dir_all(&config_dir).await?;
        fs::write(&config_path, toml::to_string(&Config::default())?).await?;
        Config::default()
    } else {
        let config = fs::read_to_string(&config_path).await?;
        toml::from_str(&config)?
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
