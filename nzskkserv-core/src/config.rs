use std::{future::Future, path::PathBuf};

use directories::ProjectDirs;
use log::warn;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::Error;

#[derive(Deserialize, Serialize)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

#[derive(Deserialize, Serialize)]
struct DictPath {
    path: String,
    encoding: Option<Encoding>,
}
#[derive(Deserialize, Serialize)]
struct DictUrl {
    url: String,
    encoding: Option<Encoding>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Dict {
    DictPath(String, Encoding),
    DictUrl(String),
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    google_ime_enable: bool,
    dicts: Vec<Dict>,
}

pub(crate) const DEFAULT_CONFIG: Config = Config {
    google_ime_enable: false,
    dicts: Vec::new(),
};

pub(crate) async fn load_config(config_dir: Option<&str>) -> Result<Config, Error> {
    let mut config_path = match config_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let project_dirs = ProjectDirs::from("", "", "nzskkserv").ok_or_else(|| {
                Error::ConfigReadError("Could not find config directory".to_string())
            })?;
            project_dirs.config_dir().to_path_buf()
        }
    };
    config_path.push("config.toml");
    let config_file = tokio::fs::read_to_string(&config_path)
        .await
        .map_err(|e| Error::IOError(e))?;
    let config: Config =
        toml::from_str(&config_file).map_err(|e| Error::ConfigReadError(e.to_string()))?;
    Ok(config)
}

pub(crate) async fn write_config(config: &Config, config_dir: Option<&str>) -> Result<(), Error> {
    let mut config_path = match config_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let project_dirs = ProjectDirs::from("", "", "nzskkserv").ok_or(
                Error::ConfigReadError("Could not find config directory".to_string()),
            )?;
            project_dirs.config_dir().to_path_buf()
        }
    };
    config_path.push("config.toml");

    let project_dirs = ProjectDirs::from("", "", "nzskkserv").ok_or(Error::ConfigWriteError(
        "Could not find config directory".to_string(),
    ))?;
    let mut config_file_path = project_dirs.config_dir().to_path_buf();
    config_file_path.push("config.toml");

    fs::create_dir_all(&config_file_path)
        .await
        .map_err(|e| Error::IOError(e))?;

    let config_text =
        toml::to_string(&config).map_err(|e| Error::ConfigWriteError(e.to_string()))?;
    fs::write(config_file_path, config_text);

    Ok(())
}
