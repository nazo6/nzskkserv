//! SKKサーバ本体

use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use log::info;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

mod codec;
pub mod error;
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
        Server {
            address,
            port,
            process: Arc::new(Mutex::new(Process {
                dicts,
                enable_google_cgi,
                encoding,
            })),
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
            let process = Arc::clone(&self.process);

            tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());
                let process = process.lock().await;
                let _ = process.process(stream).await;
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
    pub async fn set_dicts(&self, dicts: Vec<crate::Dict>) {
        let mut config = self.process.lock().await;
        config.dicts = dicts;
    }
    pub async fn set_google_cgi(&self, enable: bool) {
        let mut config = self.process.lock().await;
        config.enable_google_cgi = enable;
    }
}
