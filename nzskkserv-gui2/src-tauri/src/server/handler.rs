use std::collections::HashMap;

use nzskkserv_core::handler::{Entry, Handler};
use tracing::{info, warn};

use crate::dict_utils::DictDef;

pub struct ServerHandler {
    dict: HashMap<String, Vec<Entry>>,
    google_cgi: bool,
}

impl ServerHandler {
    pub async fn new_from_config(dict_defs: Vec<DictDef>, google_cgi: bool) -> Self {
        let mut dicts_data = Vec::new();
        for dict_def in dict_defs {
            let dict_data = dict_def.get_dict_data(false).await;
            match dict_data {
                Ok(dict_data) => {
                    if dict_data.is_empty() {
                        warn!(
                            "Dict has 0 entries: {}. Maybe url is invalid or format is wrong?",
                            dict_def.path_or_url
                        );
                        continue;
                    } else {
                        info!(
                            "Loaded {} entries from dict: {}",
                            dict_data.len(),
                            dict_def.path_or_url
                        );
                    }
                    dicts_data.push(dict_data);
                }
                Err(e) => warn!(
                    "Failed to load dict: {}, error: {}",
                    dict_def.path_or_url, e
                ),
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

        Self {
            dict: dicts_map,
            google_cgi,
        }
    }
}

impl Handler for ServerHandler {
    type Error = anyhow::Error;

    const SERVER_VERSION: &'static str = "nzskkserv/0.1.0";

    async fn resolve_word(&self, input: &str) -> Result<Vec<Entry>, Self::Error> {
        info!(nzskkserv_input = input);

        let output = match self.dict.get(input).cloned() {
            Some(o) => o,
            None => {
                if self.google_cgi {
                    fetch_google_cgi(input).await?
                } else {
                    vec![]
                }
            }
        };

        info!(nzskkserv_output = format!("{:?}", output));

        Ok(output)
    }
}
async fn fetch_google_cgi(query: &str) -> anyhow::Result<Vec<Entry>> {
    let mut alphabet_end = None;
    let query = if let Some(c) = query.chars().last() {
        if c.is_ascii_alphabetic() {
            let mut chars = query.chars();
            alphabet_end = Some(chars.next_back().unwrap());
            chars.as_str()
        } else {
            query
        }
    } else {
        query
    };

    type GoogleCgiResponse = Vec<(String, Vec<String>)>;
    let mut url = "https://www.google.com/transliterate?langpair=ja-Hira|ja&text=".to_string();
    url.push_str(&urlencoding::encode(query));
    url.push(',');
    let mut result = reqwest::get(url).await?.json::<GoogleCgiResponse>().await?;

    info!("Converted by google cgi server: {:?}", result);

    if result.is_empty() {
        anyhow::bail!("Failed to get result from google cgi");
    }
    let mut candidates = result.swap_remove(0).1;

    if let Some(c) = alphabet_end {
        candidates.iter_mut().for_each(|cand| {
            cand.push(c);
        });
    }

    let candidates = candidates
        .into_iter()
        .map(|c| Entry {
            candidate: c,
            description: None,
        })
        .collect();

    Ok(candidates)
}
