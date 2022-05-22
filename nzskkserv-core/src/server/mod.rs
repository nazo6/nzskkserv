//! SKKサーバ本体

use std::net::IpAddr;

use log::info;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::watch;

mod codec;
pub mod error;
pub mod interface;
mod process;

use error::Error;

use crate::Encoding;

use self::process::{process, Configuration};

pub struct Server {
    address: IpAddr,
    port: u16,
    configurator: watch::Sender<Configuration>,
    killer: broadcast::Sender<()>,
}

impl Server {
    /// 新しいskkサーバを作成します。
    ///
    /// dictsは(見出し語, 候補)の配列です。配列の順番が候補順になります。
    /// encodingはskkクライアントと通信するときに使う文字コードです。EUC-JPかUTF-8を使用できます。
    pub fn new(
        address: IpAddr,
        port: u16,
        dicts: Vec<crate::Dict>,
        enable_google_cgi: bool,
        encoding: Encoding,
    ) -> Self {
        let (killer, _) = broadcast::channel(1);
        let (configurator, _) = watch::channel(Configuration {
            dicts,
            enable_google_cgi,
            encoding,
        });
        Server {
            address,
            port,
            configurator,
            killer,
        }
    }
    /// skkサーバを開始します。
    pub async fn start(&self) -> Result<(), Error> {
        let kill_reciever = &mut self.killer.subscribe();
        tokio::select! {
            output = self.real_start() => output,
            _ = kill_reciever.recv() => Ok(()),
        }
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
        self.configurator.send(Configuration { dicts, ..crr }).ok()
    }
    pub async fn set_google_cgi(&self, enable: bool) -> Option<()> {
        let crr = (*self.configurator.borrow()).clone();
        self.configurator
            .send(Configuration {
                enable_google_cgi: enable,
                ..crr
            })
            .ok()
    }
}
