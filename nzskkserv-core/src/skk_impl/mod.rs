mod codec;

use codec::SkkCodec;
use futures::SinkExt;
use log::{info, warn};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

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
                        let candidates = candidates.and_then(format_candidates_str);

                        framed.send(SkkOutGoingEvent::Convert(candidates)).await
                    }
                    SkkIncomingEvent::Server => framed.send(SkkOutGoingEvent::Server).await,
                    SkkIncomingEvent::Version => framed.send(SkkOutGoingEvent::Version).await,
                    SkkIncomingEvent::Hostname => framed.send(SkkOutGoingEvent::Hostname).await,
                };

                match result {
                    Ok(()) => {}
                    Err(err) => {
                        warn!("Error occurred while processing incoming data: {:?}", err);
                    }
                }
            }
            Err(err) => {
                warn!("Error occurred while processing: {:?}", err);
            }
        }
    }
    info!("Socket closed");

    Ok(())
}

fn format_candidates_str(candidates: Vec<String>) -> Option<String> {
    let mut candidates_str = "/".to_string();
    for mut candidate in candidates {
        if candidate.is_empty() {
            continue;
        }
        let tmp = candidate.clone();
        let mut chars = tmp.chars();
        if chars.next().unwrap() == '/' {
            candidate.remove(0);
        }
        if chars.last().unwrap() == '/' {
            candidate.pop();
        }

        candidates_str.push_str(&candidate);
        candidates_str.push('/')
    }

    if candidates_str.is_empty() {
        None
    } else {
        Some(candidates_str)
    }
}
