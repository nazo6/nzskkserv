use std::net::{IpAddr, Ipv4Addr};

use handler::ServerHandler;
use nzskkserv_core::{Server, ServerConfig};

use crate::config::Config;

mod handler;

pub(super) async fn create_server(config: Config) -> Server<ServerHandler> {
    let server_config = ServerConfig {
        encoding: config.server_encoding.into(),
        address: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
        port: config.port.unwrap_or(1178),
    };

    Server::new(
        server_config,
        ServerHandler::new_from_config(config.dicts).await,
    )
}
