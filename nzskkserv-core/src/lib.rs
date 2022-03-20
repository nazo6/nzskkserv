use config::Config;
use directories::ProjectDirs;
use log::debug;
use nzskkserv_server::Candidates;
use std::{cell::RefCell, collections::HashMap, net::IpAddr, future::Future};
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
    server: Option<nzskkserv_server::Server>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Server error: {0}")]
    Server(nzskkserv_server::error::Error),
    #[error("Failed to read config: {0}")]
    ConfigRead(String),
    #[error("Failed to write config: {0}")]
    ConfigWrite(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Error occurred: {0}")]
    Other(String),
}

impl Default for Server {
    fn default() -> Self {
        Server {
            dicts: RefCell::new(Vec::new()),
            config: RefCell::new(config::DEFAULT_CONFIG),
            server: None
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
    pub async fn start(&mut self, address: IpAddr, port: u16) {
        self.server = Some(nzskkserv_server::Server::new(address, port, move |str| async move{
            debug!("Starting convertion {}", &str);
            Candidates {
                content: vec!["a".to_string()],
                anotation: Some("a".to_string())
            }
        }));
        self.server.as_ref().unwrap().start().await;
    }
    pub async fn stop(&self) -> Result<(), Error> {
        match &self.server {
            Some(server) => Ok(server.shutdown()),
            None => Err(Error::Other("Server is not started".to_string()))
        }
    }
}
