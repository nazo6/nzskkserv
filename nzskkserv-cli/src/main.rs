use anyhow::Error;
use nzskkserv_core::server::ServerConfig;
use std::net::{IpAddr, Ipv4Addr};

pub mod config;
mod dict_utils;

use config::Config;
use log::{debug, info};

use crate::config::write_config;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let config = load_config().await;

    let encoding = match config.server_encoding {
        config::Encoding::Utf8 => nzskkserv_core::Encoding::Utf8,
        config::Encoding::Eucjp => nzskkserv_core::Encoding::Eucjp,
    };
    let dicts = load_dicts(config.dicts).await;

    let server_config = ServerConfig {
        dicts,
        enable_google_cgi: config.enable_google_cgi,
        encoding,
    };
    let server = nzskkserv_core::Server::new(LOCALHOST, config.port.unwrap_or(1178), server_config);

    let start_server = || async {
        let _ = server.start().await;
        info!("Server exited")
    };

    tokio::join! {
        start_server(),
    };

    Ok(())
}

async fn load_config() -> Config {
    match config::read_config().await {
        Ok(data) => data,
        Err(e) => {
            debug!("Config load error. Falling back.: {:?}", e);
            write_config(&config::DEFAULT_CONFIG).await.unwrap();
            config::DEFAULT_CONFIG
        }
    }
}

async fn load_dicts(dicts: Vec<config::Dict>) -> Vec<nzskkserv_core::Dict> {
    let mut dicts_data: Vec<nzskkserv_core::Dict> = Vec::new();
    for dict in dicts {
        debug!("Loading dict: {:?}", dict);
        let dict_data = dict_utils::get_dict_data(dict).await;
        match dict_data {
            Ok(dict_data) => dicts_data.push(dict_data),
            Err(e) => info!("Dict load error: {:?}", e),
        }
    }
    info!("Loaded {} dicts", dicts_data.len());
    dicts_data
}
