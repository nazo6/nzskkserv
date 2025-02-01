use bytes::BytesMut;
use encoding_rs::{EUC_JP, UTF_8};
use tokio_util::codec::{Decoder, Encoder};

use crate::handler::Handler;
use crate::Encoding;
use crate::Error;

use super::SkkIncomingEvent;
use super::SkkOutGoingEvent;

pub(crate) struct SkkCodec<'a, H: Handler> {
    encoding: Encoding,
    handler: &'a H,
}

impl<'a, H: Handler> SkkCodec<'a, H> {
    pub fn new(encoding: &Encoding, handler: &'a H) -> Self {
        SkkCodec {
            encoding: encoding.clone(),
            handler,
        }
    }
}

impl<'a, H: Handler> Decoder for SkkCodec<'a, H> {
    type Item = SkkIncomingEvent;
    type Error = Error<H::Error>;
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
            let data = match command {
                '0' => Ok(Some(SkkIncomingEvent::Disconnect)),
                '1' => {
                    let content: Option<&str>;
                    // SKKクライアントによって" \n"で終わるものがあったり" "で終わるものがあったりする
                    if str.ends_with(" \n") {
                        content = str.get(1..str.len() - 2);
                    } else if str.ends_with(' ') {
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
            };

            data
        }
    }
}

impl<'a, H: Handler> Encoder<SkkOutGoingEvent> for SkkCodec<'a, H> {
    type Error = Error<H::Error>;

    fn encode(&mut self, event: SkkOutGoingEvent, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let text = match event {
            SkkOutGoingEvent::Convert(candidates) => match candidates {
                Some(candidates) => {
                    let mut str = "1".to_string();
                    str.push_str(&candidates);
                    str.push('\n');

                    str
                }
                None => "4\n".to_string(),
            },
            SkkOutGoingEvent::Server => "4\n".to_string(),
            SkkOutGoingEvent::Version => H::SERVER_VERSION.to_string(),
            SkkOutGoingEvent::Hostname => self
                .handler
                .get_hostname()
                .map_err(|e| Error::HandlerError(e))?,
        };

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
