use std::{collections::HashMap, path::PathBuf};

use directories::ProjectDirs;
use encoding_rs::{EUC_JP, UTF_8};
use tracing::warn;
use url::Url;

use anyhow::{Context, Error};

use crate::config::{DictDef, DictPath, Encoding};

pub type Dicts = HashMap<String, Vec<String>>;

pub(crate) async fn load_dicts(dicts: Vec<DictDef>) -> Dicts {
    let mut dicts_data = Vec::new();
    for dict in dicts {
        let dict_data = get_dict_data(dict).await;
        match dict_data {
            Ok(dict_data) => dicts_data.push(dict_data),
            Err(e) => warn!("Failed to load dict: {}", e),
        }
    }

    let mut dicts_map = HashMap::new();
    for dict_data in dicts_data {
        for (key, value) in dict_data {
            dicts_map.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    dicts_map
}

async fn get_dict_data(
    DictDef {
        path_or_url,
        encoding,
    }: DictDef,
) -> Result<Vec<(String, String)>, Error> {
    let dict_path = match path_or_url {
        DictPath::File { path } => path,
        DictPath::Url { url } => cache_online_dict(url).await?,
    };

    let dict_bin = tokio::fs::read(&dict_path).await?;
    let (dict_str, _, _) = match &encoding {
        Some(encoding) => match encoding {
            Encoding::Utf8 => UTF_8.decode(&dict_bin),
            Encoding::Eucjp => EUC_JP.decode(&dict_bin),
        },
        None => UTF_8.decode(&dict_bin),
    };

    let mut dict_data = vec![];
    for line in (*dict_str).lines() {
        let line = line.split_once(' ');
        if let Some(line) = line {
            dict_data.push((line.0.to_string(), line.1.to_string()));
        }
    }

    Ok(dict_data)
}

/// Return directory that contains online dict.
fn get_dict_cache_path(dict_url: &Url) -> Result<PathBuf, Error> {
    let project_dirs =
        ProjectDirs::from("", "", "nzskkserv").context("Could not find data directory")?;
    let mut data_path = project_dirs.data_dir().to_path_buf();
    data_path.push(sanitize_filename::sanitize(dict_url));
    Ok(data_path.to_str().unwrap().to_string().into())
}

async fn cache_online_dict(dict_url: Url) -> Result<PathBuf, Error> {
    let dict_path = get_dict_cache_path(&dict_url)?;
    let file = tokio::fs::File::open(&dict_path).await;
    if file.is_err() {
        let res = reqwest::get(&dict_url.to_string()).await?.bytes().await?;
        let path = PathBuf::from(&dict_path);
        tokio::fs::create_dir_all(path.parent().unwrap()).await?;
        tokio::fs::write(path, res).await?;
    }
    Ok(dict_path)
}
