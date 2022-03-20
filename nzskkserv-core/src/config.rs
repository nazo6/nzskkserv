use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::Error;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DictPath {
    pub path: String,
    pub encoding: Option<Encoding>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DictUrl {
    pub url: String,
    pub encoding: Option<Encoding>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Dict {
    DictPath(DictPath),
    DictUrl(DictUrl),
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub enable_google_cgi: bool,
    pub dicts: Vec<Dict>,
}

pub(crate) const DEFAULT_CONFIG: Config = Config {
    enable_google_cgi: false,
    dicts: Vec::new(),
};

pub(crate) async fn read_config(config_dir: Option<&str>) -> Result<Config, Error> {
    let mut config_path = match config_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let project_dirs = ProjectDirs::from("", "", "nzskkserv")
                .ok_or_else(|| Error::ConfigRead("Could not find config directory".to_string()))?;
            project_dirs.config_dir().to_path_buf()
        }
    };
    fs::create_dir_all(&config_path)
        .await
        .map_err(Error::Io)?;
    config_path.push("config.toml");

    let config_file = tokio::fs::read_to_string(&config_path)
        .await
        .map_err(Error::Io)?;
    let config: Config =
        toml::from_str(&config_file).map_err(|e| Error::ConfigRead(e.to_string()))?;
    Ok(config)
}

pub(crate) async fn write_config(config: &Config, config_dir: Option<&str>) -> Result<(), Error> {
    let mut config_path = match config_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let project_dirs = ProjectDirs::from("", "", "nzskkserv")
                .ok_or_else(|| Error::ConfigRead("Could not find config directory".to_string()))?;
            project_dirs.config_dir().to_path_buf()
        }
    };
    config_path.push("config.toml");

    let project_dirs = ProjectDirs::from("", "", "nzskkserv")
        .ok_or_else(|| Error::ConfigWrite("Could not find config directory".to_string()))?;
    let mut config_file_path = project_dirs.config_dir().to_path_buf();

    fs::create_dir_all(&config_file_path)
        .await
        .map_err(Error::Io)?;

    config_file_path.push("config.toml");

    let config_text = toml::to_string(&config).map_err(|e| Error::ConfigWrite(e.to_string()))?;
    fs::write(config_file_path, config_text).await?;

    Ok(())
}
