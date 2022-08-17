use bytes::Bytes;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid incoming command {0}")]
    InvalidIncomingCommand(String),
    #[error("Error occurred while decoding")]
    Decoding(Bytes),
    #[error("Error occurred while encoding")]
    Encoding(Bytes),
    #[error("Error occurred while parsing google cgi data")]
    GoogleCgiParse,
    #[error("Failed to set logger. Maybe already set?")]
    LoggerSet,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("Other error {0}")]
    Other(String),
}
