pub mod error;
pub mod handler;
mod skk_impl;

use std::net::IpAddr;

pub use error::Error;
use handler::Handler;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Clone)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

pub struct ServerConfig {
    pub encoding: Encoding,
    pub address: IpAddr,
    pub port: u16,
}

pub struct Server<H: Handler> {
    config: ServerConfig,
    handler: H,
}

impl<H: Handler> Server<H> {
    pub fn new(config: ServerConfig, handler: H) -> Self {
        Server { config, handler }
    }

    pub async fn start(&mut self) -> Result<(), Error<H::Error>> {
        let s = &*self;

        info!("Starting server: {}:{}", s.config.address, s.config.port);

        let listener = TcpListener::bind((self.config.address, self.config.port)).await?;

        loop {
            let (stream, socket) = listener.accept().await?;

            info!("Socket connected: {}:{}", socket.ip(), socket.port());

            if let Err(e) = skk_impl::process_skk(stream, &s.config, &s.handler).await {
                warn!("Error: {}", e);
            };
        }
    }
}
