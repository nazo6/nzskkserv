use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use anyhow::Context;
use anyhow::Error;

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
    pub server_encoding: Encoding,
    pub port: Option<u16>,
    pub dicts: Vec<Dict>,
}

pub(crate) const DEFAULT_CONFIG: Config = Config {
    enable_google_cgi: false,
    server_encoding: Encoding::Utf8,
    dicts: Vec::new(),
    port: Some(1178)
};

pub(crate) async fn read_config() -> Result<Config, Error> {
    let project_dirs = ProjectDirs::from("", "", "nzskkserv").context("No project dirs")?;
    let mut config_path = project_dirs.config_dir().to_path_buf();
    fs::create_dir_all(&config_path).await?;
    config_path.push("config.toml");

    let config_file = tokio::fs::read_to_string(&config_path).await?;
    let config: Config = toml::from_str(&config_file)?;
    Ok(config)
}

pub(crate) async fn write_config(config: &Config) -> Result<(), Error> {
    let project_dirs =
        ProjectDirs::from("", "", "nzskkserv").context("Could not find config dir.")?;
    let mut config_file_path = project_dirs.config_dir().to_path_buf();

    fs::create_dir_all(&config_file_path).await?;

    config_file_path.push("config.toml");

    let config_text = toml::to_string(&config)?;
    fs::write(config_file_path, config_text).await?;

    Ok(())
}
