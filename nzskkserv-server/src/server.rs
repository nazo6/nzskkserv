use std::net::IpAddr;
use std::sync::Arc;
use std::pin::Pin;

use futures::SinkExt;
use log::{debug, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::shutdown::Shutdown;
use crate::{
    codec::SkkCodec,
    error::Error,
    handler::Handler,
    interface::{SkkIncomingEvent, SkkOutcomingEvent},
};

pub struct Server {
    address: IpAddr,
    port: u16,
    candidates_getter: Arc<Pin<Box<dyn Handler>>>,
    notify_shutdown: broadcast::Sender<bool>,
}

impl Server {
    pub fn new<H: Handler>(address: IpAddr, port: u16, candidates_getter: H) -> Self {
        let (notify_shutdown, _) = broadcast::channel(1);
        Server {
            address,
            port,
            candidates_getter: Arc::new(Box::pin(candidates_getter)),
            notify_shutdown,
        }
    }
    pub async fn start(&self) -> Result<(), Error> {
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (stream, socket) = listener.accept().await?;
            let shutdown = Shutdown::new(self.notify_shutdown.subscribe());

            let getter = Arc::clone(&self.candidates_getter);
            tokio::spawn(async move {
                info!("Socket connected: {}:{}", socket.ip(), socket.port());
                Self::process(stream, getter, shutdown).await
            });
        }
    }
    pub fn shutdown(&self) {
        self.notify_shutdown.send(true);
    }
    async fn process(
        stream: TcpStream,
        candidates_getter: Arc<Pin<Box<dyn Handler>>>,
        mut shutdown: Shutdown,
    ) -> Result<(), Error> {
        let mut framed = Framed::new(stream, SkkCodec::new(crate::Encoding::Utf8));
        loop {
            if shutdown.is_shutdown() {
                break;
            }
            let frame = tokio::select! {
                res = framed.next() => res,
                _ = shutdown.recv() => {
                    break;
                }
            };
            match frame {
                Some(message) => {
                    let candidates_getter = Arc::clone(&candidates_getter);
                    match message {
                        Ok(data) => {
                            info!("Data incoming: {:?}", data);
                            let result = match data {
                                SkkIncomingEvent::Disconnect => {
                                    break;
                                }
                                SkkIncomingEvent::Convert(str) => {
                                    let candidates = (*candidates_getter).as_ref().apply(str).await;
                                    match candidates {
                                        Ok(candidates) => {
                                            framed
                                                .send(SkkOutcomingEvent::Convert(candidates))
                                                .await
                                        }
                                        Err(e) => Err(Error::Unknown(e.to_string())),
                                    }
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
                            match result {
                                Ok(()) => debug!("Proccessed incoming data"),
                                Err(err) => {
                                    warn!(
                                        "Error occurred while processing incoming data: {:?}",
                                        err
                                    )
                                }
                            }
                        }
                        Err(err) => {
                            warn!("Error occurred while processing: {:?}", err)
                        }
                    }
                }
                None => break,
            }
        }
        info!("socket closed");

        Ok(())
    }
}
