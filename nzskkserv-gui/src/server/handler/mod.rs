use nzskkserv_core::handler::Handler;
use tracing::info;

use crate::config::DictDef;

mod dict;

pub struct ServerHandler {
    dict: dict::Dicts,
}

impl ServerHandler {
    pub async fn new_from_config(dict_defs: Vec<DictDef>) -> Self {
        let dicts_data = dict::load_dicts(dict_defs).await;

        Self { dict: dicts_data }
    }
}

impl Handler for ServerHandler {
    type Error = anyhow::Error;

    const SERVER_VERSION: &'static str = "nzskkserv/0.1.0";

    async fn resolve_word(&self, input: &str) -> Result<Option<Vec<String>>, Self::Error> {
        info!(nzskkserv_input = input);

        let output = self.dict.get(input).cloned();

        let output_word = output.clone().map(|v| v.join("/")).unwrap_or_default();
        info!(nzskkserv_output = output_word);

        Ok(output)
    }
}
