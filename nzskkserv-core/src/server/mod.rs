//! SKKサーバ本体

use std::net::IpAddr;
use std::sync::Mutex;

use log::info;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::watch;

mod codec;
pub mod interface;
mod process;

use crate::Error;
use process::process;

#[derive(Clone)]
pub struct ServerConfig {
    pub dicts: Vec<crate::Dict>,
    pub enable_google_cgi: bool,
    pub encoding: crate::Encoding,
}

pub struct Server {
    address: IpAddr,
    port: u16,
    configurator: watch::Sender<ServerConfig>,
    killer: broadcast::Sender<()>,
    running: Mutex<bool>,
}
impl Server {
    /// 新しいskkサーバを作成します。
    ///
    /// dictsは(見出し語, 候補)の配列です。配列の順番が候補順になります。
    /// encodingはskkクライアントと通信するときに使う文字コードです。EUC-JPかUTF-8を使用できます。
    pub fn new(address: IpAddr, port: u16, config: ServerConfig) -> Self {
        let (killer, _) = broadcast::channel(1);
        let (configurator, _) = watch::channel(config);
        Server {
            address,
            port,
            configurator,
            killer,
            running: Mutex::new(false),
        }
    }
    /// skkサーバを開始します。
    pub async fn start(&self) -> Result<(), Error> {
        let kill_reciever = &mut self.killer.subscribe();
        *self.running.lock().unwrap() = true;
        let res = tokio::select! {
            output = self.real_start() => output,
            _ = kill_reciever.recv() => Ok(()),
        };
        *self.running.lock().unwrap() = false;
        res
    }
    async fn real_start(&self) -> Result<(), Error> {
        info!("Starting server: {}:{}", self.address, self.port);
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (stream, socket) = listener.accept().await?;

            let config_subscriber = self.configurator.subscribe();

            tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());

                let conf = config_subscriber.borrow().clone();

                process(stream, conf).await
            });
        }
    }
    /// skkサーバを停止するようにシグナルを送信します。実際に停止するとstartのawaitが返されます。
    pub fn stop(&self) -> Result<usize, Error> {
        self.killer
            .send(())
            .map_err(|_| Error::Other("Failed to send kill signal.".to_string()))
    }
    /// 辞書をサーバーにセットします。上書きされます。
    pub async fn set_dicts(&self, dicts: Vec<crate::Dict>) -> Option<()> {
        let crr = (*self.configurator.borrow()).clone();
        self.configurator.send(ServerConfig { dicts, ..crr }).ok()
    }
    pub async fn set_google_cgi(&self, enable: bool) -> Option<()> {
        let crr = (*self.configurator.borrow()).clone();
        self.configurator
            .send(ServerConfig {
                enable_google_cgi: enable,
                ..crr
            })
            .ok()
    }
    pub async fn update_config<F: Fn(ServerConfig) -> ServerConfig>(&self, f: F) {
        let current_cfg = self.configurator.borrow();
        f(current_cfg.clone());
    }
    pub fn get_running(&self) -> bool {
        let res = *self.running.lock().unwrap();
        res
    }
}
