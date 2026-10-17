use std::net::{IpAddr, Ipv4Addr};

use handler::ServerHandler;
use nzskkserv_core::{Server as ServerCore, ServerConfig};
use tokio::{select, sync::watch};
use tracing::{error, info};

use crate::config::Config;

use serde::{Deserialize, Serialize};

mod handler;

type Server = ServerCore<ServerHandler>;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ServerState {
    pub running: bool,
    pub config: Config,
}

pub type ServerStateController = watch::Sender<ServerState>;

pub fn start(initial_state: ServerState) -> ServerStateController {
    let (state_tx, mut state_rx) = watch::channel(initial_state.clone());

    tauri::async_runtime::spawn(async move {
        let mut prev_config = initial_state.config;

        loop {
            loop {
                if state_rx.borrow_and_update().running {
                    break;
                } else {
                    let _ = state_rx.changed().await;
                }
            }

            let new_config = state_rx.borrow_and_update().config.clone();

            if new_config != prev_config {
                let res = crate::config::write_config(&new_config).await;
                info!("Config saved: {:?}", res);
                prev_config = new_config.clone();
            }

            let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
            let mut server = create_server(new_config).await;

            tauri::async_runtime::spawn(async move {
                if ready_rx.await.is_ok() {
                    crate::ime::notify_ime().await;
                }
            });

            select! {
                res = server.start_with_ready(ready_tx) => {
                    if let Err(e) = res {
                        error!("Server exited unexpectedly: {}, waiting for 5 seconds to restart...", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
                _ = state_rx.changed() => {}
            }

            info!("Server exited.");
        }
    });

    state_tx
}

async fn create_server(config: Config) -> Server {
    let server_config = ServerConfig {
        encoding: config.server_encoding.into(),
        address: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
        port: config.port,
    };

    ServerCore::new(
        server_config,
        ServerHandler::new_from_config(config.dicts, config.enable_google_cgi).await,
    )
}
