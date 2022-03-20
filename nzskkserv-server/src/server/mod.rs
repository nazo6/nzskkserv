use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use log::{debug, info, warn};
use tokio::net::TcpListener;
use tokio::sync::broadcast::{self, Receiver};

mod codec;
mod dict;
pub(crate) mod error;
mod interface;
mod process;

use error::Error;

use self::process::Process;

type Dicts = Vec<HashMap<String, String>>;

#[derive(Clone)]
pub struct Server {
    address: IpAddr,
    port: u16,
    process: Arc<Mutex<Process>>,
}

impl Server {
    pub fn new(address: IpAddr, port: u16, dicts: Dicts, enable_google_ime: bool) -> Self {
        Server {
            address,
            port,
            process: Arc::new(Mutex::new(Process {
                dicts,
                enable_google_ime,
            })),
        }
    }
    pub async fn start(&self, mut kill: Receiver<()>) -> Result<(), Error> {
        tokio::select! {
            output = self.real_start() => output,
            _ = kill.recv() => Ok(()),
        }
    }
    async fn real_start(&self) -> Result<(), Error> {
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (stream, socket) = listener.accept().await?;
            let process = Arc::clone(&self.process);

            tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());
                let process = process.lock().await;
                let _ = process.process(stream).await;
            });
        }
    }
}
