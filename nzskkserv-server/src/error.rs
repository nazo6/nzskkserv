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
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Unknown error {0}")]
    Unknown(String),
}
