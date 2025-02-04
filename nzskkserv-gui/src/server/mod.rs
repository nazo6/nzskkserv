use std::net::{IpAddr, Ipv4Addr};

use handler::ServerHandler;
use log::{error, info, warn};
use nzskkserv_core::{Server, ServerConfig};
use tokio::{select, sync::watch};

use crate::config::Config;

mod handler;

pub type ServerController = watch::Sender<ServerState>;
pub type ServerReceiver = watch::Receiver<ServerState>;

pub struct ServerState {
    pub config: Config,
    pub running: bool,
}

/// Spawns server and return channel to change config.
pub(super) fn start(initial_config: Config) -> ServerController {
    let (server_config_sender, mut server_config_receiver) = watch::channel(ServerState {
        config: initial_config.clone(),
        running: true,
    });

    tokio::spawn(async move {
        let mut config = initial_config;

        loop {
            loop {
                {
                    let state = server_config_receiver.borrow_and_update();
                    if state.running {
                        break;
                    }
                }
                let _ = server_config_receiver.changed().await;
            }

            let server_task = async {
                info!("Starting server");
                let mut server = create_server(config.clone()).await;
                let _ = server.start().await;
            };

            let recv_task = async {
                if server_config_receiver.changed().await.is_err() {
                    error!("Server config receiver error. Sender may be dropped.");
                    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
                }
                let state = server_config_receiver.borrow_and_update();
                state.config.clone()
            };

            select! {
                _ = server_task => {},
                c = recv_task => {
                    info!("Server stopped");
                    config = c
                },
            }
        }
    });

    server_config_sender
}

pub async fn create_server(config: Config) -> Server<ServerHandler> {
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
