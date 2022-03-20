use std::{cell::RefCell, collections::HashMap, net::IpAddr};

use log::info;
use nzskkserv_server::server::Server;
use tokio::sync::broadcast;

pub mod error;
pub use error::Error;

pub struct Manager {
    server: Server,
    kill_sender: RefCell<Option<tokio::sync::broadcast::Sender<()>>>,
}

impl Manager {
    pub fn new(address: IpAddr, port: u16) -> Manager {
        let mut dict1 = HashMap::new();
        dict1.insert("あ".to_string(), "おっぱっぴー".to_string());

        let dicts = vec![dict1];

        let server = Server::new(address, port, dicts, true, nzskkserv_server::Encoding::Utf8);

        Manager {
            server,
            kill_sender: RefCell::new(None),
        }
    }
    pub async fn start(&self) {
        let (sender, reciever) = broadcast::channel(1);
        let mut kill_sender = self.kill_sender.borrow_mut();
        *kill_sender = Some(sender);
        drop(kill_sender);
        let result = self.server.start(reciever).await;
        info!("Server stopped. Status: {:?}", result)
    }
    pub async fn stop(&self) -> Result<(), Error> {
        let kill_sender = self.kill_sender.borrow();
        match &*kill_sender {
            Some(sender) => match sender.send(()) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::Other(e.to_string())),
            },
            None => Err(Error::Other("Server is not started".to_string())),
        }
    }
}
