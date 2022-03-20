use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use log::info;
use tokio::net::TcpListener;
use tokio::sync::broadcast::Receiver;

mod codec;
mod dict;
pub(crate) mod error;
mod interface;
mod process;

use error::Error;

use crate::Encoding;

use self::process::Process;

#[derive(Clone)]
pub struct Server {
    address: IpAddr,
    port: u16,
    process: Arc<Mutex<Process>>,
}

impl Server {
    pub fn new(
        address: IpAddr,
        port: u16,
        dicts: crate::Dicts,
        enable_google_cgi: bool,
        encoding: Encoding,
    ) -> Self {
        Server {
            address,
            port,
            process: Arc::new(Mutex::new(Process {
                dicts,
                enable_google_cgi,
                encoding,
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
