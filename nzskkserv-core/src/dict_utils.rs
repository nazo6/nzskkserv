use std::{path::PathBuf, collections::HashMap};

use directories::ProjectDirs;
use encoding_rs::{EUC_JP, UTF_8};

use crate::{config::{Encoding, DictPath, DictUrl}, DictData, Error};

/// Return directory that contains online dict.
fn get_dict_cache_path(dict_url: &DictUrl, data_dir: Option<&str>) -> Result<DictPath, Error> {
    let mut data_path = match data_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let project_dirs = ProjectDirs::from("", "", "nzskkserv")
                .ok_or_else(|| Error::ConfigRead("Could not find data directory".to_string()))?;
            project_dirs.data_dir().to_path_buf()
        }
    };
    data_path.push(sanitize_filename::sanitize(&dict_url.url));
    Ok(DictPath {
        path: data_path.to_str().unwrap().to_string(),
        encoding: dict_url.encoding.clone()
    })
}

async fn get_dict(dict_path: &DictPath) -> Result<DictData, Error> {
    let dict_bin = tokio::fs::read(&dict_path.path).await.map_err(Error::Io)?;
    let (dict_str, _, _) = match &dict_path.encoding {
        Some(encoding) => {
            match encoding {
                Encoding::Utf8 => UTF_8.decode(&dict_bin),
                Encoding::Eucjp => EUC_JP.decode(&dict_bin),
            }
        }
        None => {
            UTF_8.decode(&dict_bin)
        }
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
    dict: &crate::Dict,
    data_dir: Option<&str>,
) -> Result<DictData, Error> {
    match dict {
        crate::Dict::DictPath(dict_path) => {
            get_dict(dict_path).await
        },
        crate::Dict::DictUrl(dict_url) => {
            let dict_path = get_dict_cache_path(dict_url, data_dir)?;
            get_dict(&dict_path).await
        }
    }
}

// Fetch DictUrl from url and save to cache folder.
// pub async fn save_online_dict(dict_url: &crate::Dict::DictUrl) -> Result<DictData, Error> {}
