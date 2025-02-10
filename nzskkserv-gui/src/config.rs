use std::path::PathBuf;
use std::sync::LazyLock;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::info;

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

impl std::fmt::Display for DictPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DictPath::File { path } => write!(f, "{}", path.to_string_lossy()),
            DictPath::Url { url } => write!(f, "{}", url),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DictFormat {
    Skk,
    Mozc,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DictDef {
    #[serde(flatten)]
    pub path_or_url: DictPath,
    #[serde(default = "default_encoding")]
    pub encoding: Encoding,
    #[serde(default = "default_format")]
    pub format: DictFormat,
}

fn default_encoding() -> Encoding {
    Encoding::Utf8
}
fn default_format() -> DictFormat {
    DictFormat::Skk
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

pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let project_dirs = ProjectDirs::from("", "", "nzskkserv").expect("No project dirs");
    let config_dir = project_dirs.config_dir().to_path_buf();
    config_dir.join("config.toml")
});

pub(crate) async fn load_config() -> Result<Config> {
    let config: Config = if !CONFIG_PATH.exists() {
        info!("Config file not found. Creating default config file.");
        fs::create_dir_all(&CONFIG_PATH.parent().unwrap()).await?;
        fs::write(&*CONFIG_PATH, toml::to_string(&Config::default())?).await?;
        Config::default()
    } else {
        let config = fs::read_to_string(&*CONFIG_PATH).await?;
        toml::from_str(&config)?
    };

    Ok(config)
}

pub(crate) async fn write_config(config: &Config) -> Result<()> {
    fs::create_dir_all(CONFIG_PATH.parent().unwrap()).await?;

    let config_text = toml::to_string(&config)?;
    fs::write(&*CONFIG_PATH, config_text).await?;

    Ok(())
}
