use std::{collections::HashMap, path::PathBuf};

use directories::ProjectDirs;
use encoding_rs::{EUC_JP, UTF_8};
use nzskkserv_core::handler::Entry;
use tracing::{info, warn};
use url::Url;

use anyhow::{Context, Error};

use crate::config::{DictDef, DictFormat, DictPath, Encoding};

mod mozc;
mod skk;

pub type Dicts = HashMap<String, Vec<Entry>>;

pub(crate) async fn load_dicts(dicts: Vec<DictDef>) -> Dicts {
    let mut dicts_data = Vec::new();
    for dict in dicts {
        let dict_data = get_dict_data(dict.clone()).await;
        match dict_data {
            Ok(dict_data) => {
                if dict_data.is_empty() {
                    warn!(
                        "Dict has 0 entries: {}. Maybe url is invalid or format is wrong?",
                        dict.path_or_url
                    );
                    continue;
                } else {
                    info!(
                        "Loaded {} entries from dict: {}",
                        dict_data.len(),
                        dict.path_or_url
                    );
                }
                dicts_data.push(dict_data);
            }
            Err(e) => warn!("Failed to load dict: {}, error: {}", dict.path_or_url, e),
        }
    }

    let dicts_count = dicts_data.len();
    let mut dicts_map = HashMap::new();
    for dict_data in dicts_data {
        for (key, mut entries) in dict_data {
            dicts_map
                .entry(key)
                .or_insert_with(Vec::new)
                .append(&mut entries);
        }
    }

    info!("Loaded {} keys from {} dicts", dicts_map.len(), dicts_count);

    dicts_map
}

async fn get_dict_data(
    DictDef {
        path_or_url,
        encoding,
        format,
    }: DictDef,
) -> Result<Vec<(String, Vec<Entry>)>, Error> {
    let dict_path = match path_or_url {
        DictPath::File { path } => path,
        DictPath::Url { url } => cache_online_dict(url).await?,
    };

    let dict_bin = tokio::fs::read(&dict_path).await?;
    let (dict_str, _, _) = match &encoding {
        Encoding::Utf8 => UTF_8.decode(&dict_bin),
        Encoding::Eucjp => EUC_JP.decode(&dict_bin),
    };

    let dicts = match format {
        DictFormat::Skk => skk::parse_skk_dict(&dict_str),
        DictFormat::Mozc => mozc::parse_mozc_dict(&dict_str),
    };

    Ok(dicts)
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
