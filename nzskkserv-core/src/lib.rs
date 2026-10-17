pub mod error;
pub mod handler;
mod skk_impl;

use std::{net::IpAddr, sync::Arc};

pub use error::Error;
use handler::Handler;
use tokio::{net::TcpListener, task::JoinHandle};
use tracing::{info, warn};

#[derive(Clone)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub encoding: Encoding,
    pub address: IpAddr,
    pub port: u16,
}

pub struct Server<H: Handler> {
    config: ServerConfig,
    handler: Arc<H>,
}

impl<H: Handler> Server<H> {
    pub fn new(config: ServerConfig, handler: H) -> Self {
        Server {
            config,
            handler: Arc::new(handler),
        }
    }

    pub async fn start(&mut self) -> Result<(), Error<H::Error>> {
        let (ready_tx, _) = tokio::sync::oneshot::channel();
        self.start_with_ready(ready_tx).await
    }

    pub async fn start_with_ready(
        &mut self,
        ready_tx: tokio::sync::oneshot::Sender<()>,
    ) -> Result<(), Error<H::Error>> {
        let s = &*self;

        info!("Starting server: {}:{}", s.config.address, s.config.port);

        let listener = TcpListener::bind((self.config.address, self.config.port)).await?;
        let _ = ready_tx.send(());

        let mut aborter = TaskAborter { tasks: Vec::new() };
        loop {
            let (stream, socket) = listener.accept().await?;
            let config = s.config.clone();
            let handler = s.handler.clone();

            aborter.tasks.push(tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());

                if let Err(e) = skk_impl::process_skk(stream, &config, &*handler).await {
                    warn!("Error: {}", e);
                };
            }));
        }
    }
}

struct TaskAborter {
    tasks: Vec<JoinHandle<()>>,
}

impl Drop for TaskAborter {
    fn drop(&mut self) {
        info!("Aborting {} connections", self.tasks.len());
        for task in self.tasks.drain(..) {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::Entry;
    use std::net::Ipv4Addr;

    struct DummyHandler;
    impl Handler for DummyHandler {
        type Error = std::convert::Infallible;
        const SERVER_VERSION: &'static str = "test";
        async fn resolve_word(&self, _input: &str) -> Result<Vec<Entry>, Self::Error> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_server_start_with_ready() {
        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
        let config = ServerConfig {
            encoding: Encoding::Utf8,
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 0, // OS assigns an available ephemeral port
        };
        let mut server = Server::new(config, DummyHandler);

        let handle = tokio::spawn(async move {
            let _ = server.start_with_ready(ready_tx).await;
        });

        // The ready signal must be received successfully
        assert!(ready_rx.await.is_ok());

        handle.abort();
    }
}

