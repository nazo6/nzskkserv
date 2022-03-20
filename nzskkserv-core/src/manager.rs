use std::{cell::RefCell, collections::HashMap, net::IpAddr};

use log::info;
use nzskkserv_server::server::Server;
use tokio::sync::broadcast;

use crate::config;
use crate::config::write_config;
use crate::dict_utils;
use crate::Error;

pub struct Manager {
    server: Server,
    kill_sender: RefCell<Option<tokio::sync::broadcast::Sender<()>>>,
    config: RefCell<config::Config>,
}

impl Manager {
    pub fn new(address: IpAddr, port: u16) -> Manager {
        let server = Server::new(
            address,
            port,
            Vec::new(),
            false,
            nzskkserv_server::Encoding::Utf8,
        );

        Manager {
            server,
            kill_sender: RefCell::new(None),
            config: RefCell::new(config::Config {
                enable_google_cgi: false,
                dicts: Vec::new(),
            }),
        }
    }
    /// Load config file, read dicts and set them.
    pub async fn load_config(&self, config_dir: Option<&str>) {
        let config_data = match config::read_config(config_dir).await {
            Ok(data) => data,
            Err(e) => {
                info!("Error occurred while loading config: {:?}", e);
                config::DEFAULT_CONFIG
            }
        };
        let mut config = self.config.borrow_mut();
        *config = config_data;
        drop(config);
        self.load_dicts().await;
    }
    async fn load_dicts(&self) {
        let mut dicts_data: Vec<HashMap<String, String>> = Vec::new();
        let dicts = &self.config.borrow().dicts;
        for dict in dicts {
            let dict_data = dict_utils::get_dict_data(dict, None).await;
            if let Ok(dict_data) = dict_data {
                dicts_data.push(dict_data);
            }
        }
        self.server.set_dicts(dicts_data).await;
    }
    async fn write_config(&self, config_dir: Option<&str>) -> Result<(), Error> {
        let config = self.config.borrow();
        write_config(&config, config_dir).await
    }
    pub async fn start(&self) {
        let (sender, reciever) = broadcast::channel(1);
        let mut kill_sender = self.kill_sender.borrow_mut();
        *kill_sender = Some(sender);
        drop(kill_sender);
        let result = self.server.start(reciever).await;
        info!("Server stopped. Status: {:?}", result)
    }
    pub async fn stop(&self) -> Result<(), Error> {
        let kill_sender = self.kill_sender.borrow();
        match &*kill_sender {
            Some(sender) => match sender.send(()) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::Other(e.to_string())),
            },
            None => Err(Error::Other("Server is not started".to_string())),
        }
    }
}

impl Manager {
    pub async fn set_google_cgi(&self, enabled: bool) {
        self.server.set_google_cgi(enabled).await;
        {
            let mut config = self.config.borrow_mut();
            config.enable_google_cgi = enabled;
        }
        info!("Updating config file");
        self.write_config(None).await;
    }
    pub async fn set_dicts(&self, dicts: Vec<crate::Dict>) {
        {
            let mut config = self.config.borrow_mut();
            config.dicts = dicts;
        }
        info!("Loading dicts and syncing to server");
        self.load_dicts().await;
        info!("Updating config file");
        self.write_config(None).await;
    }
}
