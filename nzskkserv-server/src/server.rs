use std::future::Future;
use std::net::IpAddr;

use futures::SinkExt;
use log::{debug, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

mod codec;
pub mod interface;

use codec::SkkCodec;
use interface::{SkkIncomingEvent, SkkOutcomingEvent};

use crate::{error::Error, server::interface::Candidates};

type CandidatesGetter = fn(String) -> dyn Future<Output = Candidates>;

pub struct Server {
    address: IpAddr,
    port: u16,
    candidates_getter: CandidatesGetter,
}

impl Server {
    pub fn new(address: IpAddr, port: u16, candidates_getter: CandidatesGetter) -> Server {
        Server {
            address,
            port,
            candidates_getter,
        }
    }
    pub async fn start(self) -> Result<(), Error> {
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (stream, socket) = listener.accept().await?;

            tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());
                Self::process(stream, self.candidates_getter).await
            });
        }
    }
    async fn process(stream: TcpStream, candidates_getter: CandidatesGetter) -> Result<(), Error> {
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
                            let candidates = candidates_getter(str);
                            framed.send(SkkOutcomingEvent::Convert()).await
                        }
                        SkkIncomingEvent::Server => framed.send(SkkOutcomingEvent::Server).await,
                        SkkIncomingEvent::Version => framed.send(SkkOutcomingEvent::Version).await,
                        SkkIncomingEvent::Hostname => {
                            framed.send(SkkOutcomingEvent::Hostname).await
                        }
                    };
                    match result {
                        Ok(()) => debug!("Proccessed incoming data"),
                        Err(err) => {
                            warn!("Error occurred while processing incoming data: {:?}", err)
                        }
                    }
                }
                Err(err) => {
                    warn!("Error occurred while processing: {:?}", err)
                }
            }
        }
        println!("socket closed");

        Ok(())
    }
}
