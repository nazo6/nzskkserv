use bytes::BytesMut;
use encoding_rs::{EUC_JP, UTF_8};
use log::info;
use tokio_util::codec::{Decoder, Encoder};

use super::error::Error;
use crate::Encoding;

use super::interface::{SkkIncomingEvent, SkkOutcomingEvent};

pub(crate) struct SkkCodec {
    encoding: Encoding,
}

impl SkkCodec {
    pub fn new(encoding: &Encoding) -> SkkCodec {
        SkkCodec {
            encoding: encoding.clone(),
        }
    }
}

impl Decoder for SkkCodec {
    type Item = SkkIncomingEvent;
    type Error = Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let src = if !src.is_empty() {
            let len = src.len();
            src.split_to(len)
        } else {
            return Ok(None);
        };
        let (cow, _, had_errors) = match self.encoding {
            Encoding::Utf8 => UTF_8.decode(&src),
            Encoding::Eucjp => EUC_JP.decode(&src),
        };
        if had_errors {
            Err(Error::Decoding(src.freeze()))
        } else {
            let str = cow.to_string();
            let command = &str.chars().next();
            let command = match command {
                Some(command) => command,
                None => return Err(Error::InvalidIncomingCommand(str)),
            };
            match command {
                '0' => Ok(Some(SkkIncomingEvent::Disconnect)),
                '1' => {
                    let content: Option<&str>;
                    if str.ends_with(" \n") {
                        content = str.get(1..str.len() - 2);
                    } else if str.ends_with(" \n") {
                        content = str.get(1..str.len() - 1);
                    } else {
                        content = None;
                    }
                    match content {
                        Some(content) => Ok(Some(SkkIncomingEvent::Convert(content.to_string()))),
                        None => Err(Error::InvalidIncomingCommand(str.to_string())),
                    }
                }
                '2' => Ok(Some(SkkIncomingEvent::Version)),
                '3' => Ok(Some(SkkIncomingEvent::Hostname)),
                '4' => Ok(Some(SkkIncomingEvent::Server)),
                _ => Err(Error::InvalidIncomingCommand(str)),
            }
        }
    }
}

impl Encoder<SkkOutcomingEvent> for SkkCodec {
    type Error = Error;

    fn encode(&mut self, event: SkkOutcomingEvent, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let text = match event {
            SkkOutcomingEvent::Convert(candidates) => match candidates {
                Some(candidates) => {
                    let mut str = "1".to_string();
                    str.push_str(&candidates);
                    str.push('\n');

                    str
                }
                None => "4\n".to_string(),
            },
            SkkOutcomingEvent::Server => "4\n".to_string(),
            SkkOutcomingEvent::Version => "nzskkserv-server/0.1.0 ".to_string(),
            SkkOutcomingEvent::Hostname => " ".to_string(),
        };
        info!("Responsing: {:?}", &text);
        let (bytes, _, _) = match self.encoding {
            Encoding::Utf8 => UTF_8.encode(&text),
            Encoding::Eucjp => EUC_JP.encode(&text),
        };

        let bytes = bytes.to_vec();

        dst.reserve(bytes.len());
        dst.extend_from_slice(&bytes);

        Ok(())
    }
}
