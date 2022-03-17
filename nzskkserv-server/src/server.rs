mod codec;
mod interface;

use anyhow::Result;
use std::net::IpAddr;
use tokio_util::codec::{BytesCodec, Framed};

use bytes::Bytes;
use futures::SinkExt;

use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;

use codec::SkkCodec;

use crate::server::interface::{SkkIncomingEvent, SkkOutcomingEvent};

pub struct Server {
    address: IpAddr,
    port: u16,
}

impl Server {
    pub fn new(address: IpAddr, port: u16) -> Server {
        Server { address, port }
    }
    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                println!("Connected");
                process(socket).await
            });
        }
    }
}

async fn process(stream: TcpStream) {
    let mut framed = Framed::new(stream, SkkCodec::new(crate::Encoding::Utf8));
    while let Some(message) = framed.next().await {
        match message {
            Ok(data) => {
                println!("{:?}", data);
                let result = match data {
                    SkkIncomingEvent::Disconnect => {
                        break;
                    }
                    SkkIncomingEvent::Convert(str) => {
                        framed.send(SkkOutcomingEvent::Convert(
                            vec!["さんぷる".to_string()]
                        )).await
                    }
                    SkkIncomingEvent::Server => {
                        framed.send(SkkOutcomingEvent::Server).await
                    }
                    SkkIncomingEvent::Version => {
                        framed.send(SkkOutcomingEvent::Version).await
                    }
                    SkkIncomingEvent::Hostname => {
                        framed.send(SkkOutcomingEvent::Hostname).await
                    }
                };
            }
            Err(err) => println!("Socket closed with error: {:?}", err),
        }
    }
    println!("socket closed");
}
