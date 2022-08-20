use std::{collections::HashMap, path::PathBuf};

use directories::ProjectDirs;
use encoding_rs::{EUC_JP, UTF_8};
use nzskkserv_core::Dict;

use crate::config::{DictConf, DictPath, DictUrl, Encoding};
use anyhow::{Context, Error};

pub(crate) async fn load_dicts(dicts: Vec<DictConf>) -> Vec<Dict> {
    let mut dicts_data: Vec<nzskkserv_core::Dict> = Vec::new();
    for dict in dicts {
        let dict_data = get_dict_data(dict).await;
        match dict_data {
            Ok(dict_data) => dicts_data.push(dict_data),
            Err(e) => (),
        }
    }
    dicts_data
}

/// Return directory that contains online dict.
fn get_dict_cache_path(dict_url: &DictUrl) -> Result<DictPath, Error> {
    let project_dirs =
        ProjectDirs::from("", "", "nzskkserv").context("Could not find data directory")?;
    let mut data_path = project_dirs.data_dir().to_path_buf();
    data_path.push(sanitize_filename::sanitize(&dict_url.url));
    Ok(DictPath {
        path: data_path.to_str().unwrap().to_string(),
        encoding: dict_url.encoding.clone(),
    })
}

async fn get_dict(dict_path: &DictPath) -> Result<Dict, Error> {
    let dict_bin = tokio::fs::read(&dict_path.path).await?;
    let (dict_str, _, _) = match &dict_path.encoding {
        Some(encoding) => match encoding {
            Encoding::Utf8 => UTF_8.decode(&dict_bin),
            Encoding::Eucjp => EUC_JP.decode(&dict_bin),
        },
        None => UTF_8.decode(&dict_bin),
    };

    let mut dict_data: HashMap<String, String> = HashMap::new();
    for line in (*dict_str).lines() {
        let line = line.split_once(' ');
        if let Some(line) = line {
            dict_data.insert(line.0.to_string(), line.1.to_string());
        }
    }

    Ok(dict_data)
}

/// Read dict from file and return its data.
///
/// If Dict::DictUrl is passed, this function tries to read data from cache.
/// If cache is not available, error will be returned
pub(crate) async fn get_dict_data(
    dict: crate::config::DictConf,
) -> Result<nzskkserv_core::Dict, Error> {
    let dict_path = match dict {
        DictConf::DictPath(dict_path) => dict_path,
        DictConf::DictUrl(dict_url) => cache_online_dict(&dict_url).await?,
    };
    get_dict(&dict_path).await
}

/// Fetch DictUrl from url and save to cache folder.
/// If cache already exists, do nothing.
/// Return cache folder path.
pub(crate) async fn cache_online_dict(dict_url: &DictUrl) -> Result<DictPath, Error> {
    let dict_path = get_dict_cache_path(dict_url)?;
    let file = tokio::fs::File::open(&dict_path.path).await;
    if file.is_err() {
        let res = reqwest::get(&dict_url.url).await?.bytes().await?;
        let path = PathBuf::from(&dict_path.path);
        tokio::fs::create_dir_all(path.parent().unwrap()).await?;
        tokio::fs::write(path, res).await?;
    }
    Ok(dict_path)
}
