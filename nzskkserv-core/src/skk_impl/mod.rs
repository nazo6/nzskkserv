mod codec;

use codec::SkkCodec;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{info, warn};

use crate::{handler::Handler, Error};

#[derive(Debug, Clone)]
pub enum SkkIncomingEvent {
    /// 0
    Disconnect,
    /// 1
    Convert(String),
    /// 2
    Version,
    /// 3
    Hostname,
    /// 4
    Server,
}

#[derive(Debug, Clone)]
pub enum SkkOutGoingEvent {
    Convert(Option<String>),
    Version,
    Hostname,
    Server,
}

use super::ServerConfig;

pub(crate) async fn process_skk<H: Handler>(
    stream: TcpStream,
    config: &ServerConfig,
    handler: &H,
) -> Result<(), Error<H::Error>> {
    let mut framed = Framed::new(stream, SkkCodec::new(&config.encoding, handler));
    while let Some(message) = framed.next().await {
        match message {
            Ok(data) => {
                let result = match data {
                    SkkIncomingEvent::Disconnect => {
                        break;
                    }
                    SkkIncomingEvent::Convert(str) => {
                        let Ok(candidates) = handler.resolve_word(&str).await else {
                            continue;
                        };
                        let candidates_str = if candidates.is_empty() {
                            None
                        } else {
                            let mut str = "/".to_string();
                            candidates.iter().for_each(|c| {
                                str.push_str(&c.candidate);
                                if let Some(d) = &c.description {
                                    str.push(';');
                                    str.push_str(d);
                                }
                                str.push('/');
                            });

                            Some(str)
                        };

                        framed.send(SkkOutGoingEvent::Convert(candidates_str)).await
                    }
                    SkkIncomingEvent::Server => framed.send(SkkOutGoingEvent::Server).await,
                    SkkIncomingEvent::Version => framed.send(SkkOutGoingEvent::Version).await,
                    SkkIncomingEvent::Hostname => framed.send(SkkOutGoingEvent::Hostname).await,
                };

                match result {
                    Ok(()) => {}
                    Err(err) => {
                        warn!("Error occurred while processing incoming data: {}", err);
                    }
                }
            }
            Err(err) => {
                warn!("Error occurred while processing: {}", err);
            }
        }
    }
    info!("Socket closed");

    Ok(())
}
