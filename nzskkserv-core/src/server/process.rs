use std::collections::HashMap;

use futures::SinkExt;
use log::{debug, info, warn};
use tokio::net::TcpStream;
use tokio::sync::watch::Receiver;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::{
    server::{
        codec::SkkCodec,
        interface::{SkkIncomingEvent, SkkOutcomingEvent},
    },
    Error,
};

#[derive(Clone)]
pub(crate) struct Configuration {
    pub dicts: Vec<HashMap<String, String>>,
    pub enable_google_cgi: bool,
    pub encoding: crate::Encoding,
}

pub(crate) async fn process(
    stream: TcpStream,
    config: Configuration,
) -> Result<(), super::error::Error> {
    let mut framed = Framed::new(stream, SkkCodec::new(&config.encoding));
    while let Some(message) = framed.next().await {
        match message {
            Ok(data) => {
                info!("Data incoming: {:?}", data);
                let result = match data {
                    SkkIncomingEvent::Disconnect => {
                        break;
                    }
                    SkkIncomingEvent::Convert(str) => {
                        let candidates = convert(&str, &config).await;
                        framed.send(SkkOutcomingEvent::Convert(candidates)).await
                    }
                    SkkIncomingEvent::Server => framed.send(SkkOutcomingEvent::Server).await,
                    SkkIncomingEvent::Version => framed.send(SkkOutcomingEvent::Version).await,
                    SkkIncomingEvent::Hostname => framed.send(SkkOutcomingEvent::Hostname).await,
                };
                match result {
                    Ok(()) => {}
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
    info!("Socket closed");

    Ok(())
}
async fn convert(str: &str, config: &Configuration) -> Option<String> {
    let mut candidates: Vec<String> = Vec::new();
    for dict in &config.dicts {
        let value = dict.get(str);
        if let Some(value) = value {
            candidates.push(value.to_string())
        }
    }

    if candidates.is_empty() && config.enable_google_cgi {
        if let Ok(candidate) = fetch_google_cgi(str).await {
            candidates.push(candidate);
        }
    }

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
async fn fetch_google_cgi(query: &str) -> Result<String, Error> {
    info!("Querying to google cgi server");
    type GoogleCgiResponse = Vec<(String, Vec<String>)>;
    let mut url = "http://www.google.com/transliterate?langpair=ja-Hira|ja&text=".to_string();
    url.push_str(&urlencoding::encode(query));
    url.push(',');
    let result = reqwest::get(url).await?.json::<GoogleCgiResponse>().await?;

    debug!("Got response from google cgi server: {:?}", result);

    let candidates = &result.get(0).ok_or(Error::GoogleCgiParse)?.1;

    let mut candidate_str = "/".to_string();
    candidates.iter().for_each(|candidate| {
        candidate_str.push_str(candidate);
        candidate_str.push('/')
    });

    Ok(candidate_str)
}
