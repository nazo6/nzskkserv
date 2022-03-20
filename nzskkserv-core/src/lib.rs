use config::Config;
use directories::ProjectDirs;
use std::{cell::RefCell, collections::HashMap};
use thiserror::Error;
use tokio::fs;

mod config;

pub use nzskkserv_server::Encoding;

struct Dict {
    contents: HashMap<String, String>,
    name: String,
}

pub struct Server {
    dicts: RefCell<Vec<Dict>>,
    config: RefCell<Config>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Server error: {0}")]
    ServerError(nzskkserv_server::error::Error),
    #[error("Failed to read config: {0}")]
    ConfigReadError(String),
    #[error("Failed to write config: {0}")]
    ConfigWriteError(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl Default for Server {
    fn default() -> Self {
        Server {
            dicts: RefCell::new(Vec::new()),
            config: RefCell::new(config::DEFAULT_CONFIG),
        }
    }
}

impl Server {
    pub async fn load(&self, config_dir: Option<&str>) -> Result<(), Error> {
        let loaded_config = config::load_config(config_dir).await?;

        let mut config = self.config.borrow_mut();
        *config = loaded_config;

        Ok(())
    }
}
