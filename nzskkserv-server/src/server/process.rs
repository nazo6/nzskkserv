use std::collections::HashMap;

use futures::SinkExt;
use log::{debug, info, warn};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::server::{
    codec::SkkCodec,
    interface::{Candidates, SkkIncomingEvent, SkkOutcomingEvent},
};

pub(crate) struct Process {
    pub dicts: Vec<HashMap<String, String>>,
    pub enable_google_ime: bool,
}

impl Process {
    pub async fn process(&self, stream: TcpStream) -> Result<(), super::error::Error> {
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
                            let candidates = self.convert(str).await;
                            framed.send(SkkOutcomingEvent::Convert(candidates)).await
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
        info!("socket closed");

        Ok(())
    }
    async fn convert(&self, str: String) -> Candidates {
        debug!("{:?}", self.dicts.len());
        Candidates {
            content: vec!["a".to_string()],
            anotation: Some("nzs".to_string()),
        }
    }
}
