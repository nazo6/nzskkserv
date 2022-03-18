use std::net::IpAddr;

use log::{info, warn, debug};

use futures::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

mod codec;
mod interface;

use codec::SkkCodec;
use interface::{SkkIncomingEvent, SkkOutcomingEvent};

use crate::{error::Error, server::interface::Candidates};

pub struct Server {
    address: IpAddr,
    port: u16,
}

impl Server {
    pub fn new(address: IpAddr, port: u16) -> Server {
        Server { address, port }
    }
    pub async fn start(self) -> Result<(), Error> {
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (stream, socket) = listener.accept().await?;

            tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());
                process(stream).await
            });
        }
    }
}

async fn process(stream: TcpStream) -> Result<(), Error> {
    let mut framed = Framed::new(stream, SkkCodec::new(crate::Encoding::Utf8));
    while let Some(message) = framed.next().await {
        match message {
            Ok(data) => {
                info!("Data incoming: {:?}", data);
                let result = match data {
                    SkkIncomingEvent::Disconnect => {
                        break;
                    }
                    SkkIncomingEvent::Convert(str) => {
                        framed
                            .send(SkkOutcomingEvent::Convert(Candidates{ 
                                content: vec!["さんぷる".to_string()],
                                anotation: Some("ns".to_string())
                            }))
                            .await
                    }
                    SkkIncomingEvent::Server => framed.send(SkkOutcomingEvent::Server).await,
                    SkkIncomingEvent::Version => framed.send(SkkOutcomingEvent::Version).await,
                    SkkIncomingEvent::Hostname => framed.send(SkkOutcomingEvent::Hostname).await,
                };
                match result {
                    Ok(()) => debug!("Proccessed incoming data"),
                    Err(err) => warn!("Error occurred while processing incoming data: {:?}", err),
                }
            }
            Err(err) => {
                warn!("Error occurred while processing: {:?}", err)
            },
        }
    }
    println!("socket closed");

    Ok(())
}
