use std::path::PathBuf;

use directories::ProjectDirs;
use encoding_rs::{EUC_JP, UTF_8};
use nzskkserv_core::handler::Entry;
use serde::{Deserialize, Serialize};
use url::Url;

use anyhow::{Context, Error};

use crate::config::Encoding;

mod mozc;
mod skk;

/// Definition of dictionary location and format
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

impl DictDef {
    pub(crate) async fn get_dict_data(
        &self,
        update_cache: bool,
    ) -> Result<Vec<(String, Vec<Entry>)>, Error> {
        let dict_path = match &self.path_or_url {
            DictPath::File { path } => path,
            DictPath::Url { url } => &url.cache_and_get(update_cache).await?,
        };

        let dict_bin = tokio::fs::read(&dict_path).await?;
        let (dict_str, _, _) = match &self.encoding {
            Encoding::Utf8 => UTF_8.decode(&dict_bin),
            Encoding::Eucjp => EUC_JP.decode(&dict_bin),
        };

        let dicts = match self.format {
            DictFormat::Skk => skk::parse_skk_dict(&dict_str),
            DictFormat::Mozc => mozc::parse_mozc_dict(&dict_str),
        };

        Ok(dicts)
    }

    pub(crate) fn get_path_url_str(&self) -> String {
        match &self.path_or_url {
            DictPath::File { path } => path.to_string_lossy().to_string(),
            DictPath::Url { url } => url.0.to_string(),
        }
    }
    pub(crate) fn to_type_str(&self) -> String {
        match &self.path_or_url {
            DictPath::File { .. } => "File".to_string(),
            DictPath::Url { .. } => "Url".to_string(),
        }
    }

    pub(crate) fn set_path_url(&mut self, str: &str) -> anyhow::Result<()> {
        match &mut self.path_or_url {
            DictPath::File { path } => {
                *path = PathBuf::from(str);
            }
            DictPath::Url { url } => {
                *url = DictUrl(Url::parse(str)?);
            }
        }
        Ok(())
    }

    pub(crate) fn set_type(&mut self, new_source_type: &str) -> anyhow::Result<()> {
        match (self.path_or_url.clone(), new_source_type) {
            (DictPath::File { .. }, "Url") => {
                self.path_or_url = DictPath::Url {
                    url: DictUrl(Url::parse("http://example.com").unwrap()),
                };
            }
            (DictPath::Url { .. }, "File") => {
                self.path_or_url = DictPath::File {
                    path: PathBuf::new(),
                };
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DictFormat {
    Skk,
    Mozc,
}

impl DictFormat {
    pub(crate) fn to_str(&self) -> String {
        match self {
            DictFormat::Skk => "Skk".to_string(),
            DictFormat::Mozc => "Mozc".to_string(),
        }
    }
    pub(crate) fn from_str(str: &str) -> Self {
        match str {
            "Skk" => DictFormat::Skk,
            "Mozc" => DictFormat::Mozc,
            _ => DictFormat::Skk,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DictUrl(pub Url);

impl DictUrl {
    /// Return file path of online dict.
    pub(crate) fn get_cache_path(&self) -> Result<PathBuf, Error> {
        let project_dirs =
            ProjectDirs::from("", "", "nzskkserv").context("Could not find data directory")?;
        let mut data_path = project_dirs.data_dir().to_path_buf();
        data_path.push(sanitize_filename::sanitize(&self.0));
        Ok(data_path.to_str().unwrap().to_string().into())
    }

    /// Get cached file path of url. If not downloaded, automatically download from url
    ///
    /// * `force`: If true, always fetch from url and overwrites cache
    pub(crate) async fn cache_and_get(&self, force: bool) -> Result<PathBuf, Error> {
        let dict_path = self.get_cache_path()?;
        let file = tokio::fs::File::open(&dict_path).await;
        if file.is_err() || force {
            let res = reqwest::get(self.0.clone()).await?.bytes().await?;
            let path = PathBuf::from(&dict_path);
            tokio::fs::create_dir_all(path.parent().unwrap()).await?;
            tokio::fs::write(path, res).await?;
        }
        Ok(dict_path)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DictPath {
    File { path: PathBuf },
    Url { url: DictUrl },
}

impl std::fmt::Display for DictPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DictPath::File { path } => write!(f, "{}", path.to_string_lossy()),
            DictPath::Url { url } => write!(f, "{}", url.0),
        }
    }
}
