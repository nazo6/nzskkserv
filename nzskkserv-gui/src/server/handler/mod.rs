use nzskkserv_core::handler::Handler;
use tracing::info;

use crate::config::DictDef;

mod dict;

pub struct ServerHandler {
    dict: dict::Dicts,
    google_cgi: bool,
}

impl ServerHandler {
    pub async fn new_from_config(dict_defs: Vec<DictDef>, google_cgi: bool) -> Self {
        let dicts_data = dict::load_dicts(dict_defs).await;

        Self {
            dict: dicts_data,
            google_cgi,
        }
    }
}

impl Handler for ServerHandler {
    type Error = anyhow::Error;

    const SERVER_VERSION: &'static str = "nzskkserv/0.1.0";

    async fn resolve_word(&self, input: &str) -> Result<Option<Vec<String>>, Self::Error> {
        info!(nzskkserv_input = input);

        let output = match self.dict.get(input).cloned() {
            Some(o) => Some(o),
            None => {
                if self.google_cgi {
                    Some(fetch_google_cgi(input).await?)
                } else {
                    None
                }
            }
        };

        let output_word = output.clone().map(|v| v.join("/")).unwrap_or_default();
        info!(nzskkserv_output = output_word);

        Ok(output)
    }
}
async fn fetch_google_cgi(query: &str) -> anyhow::Result<Vec<String>> {
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

    Ok(candidates)
}
