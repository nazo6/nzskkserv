use bytes::Bytes;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<HandlerError: std::error::Error> {
    #[error("Invalid incoming command: {0}")]
    InvalidIncomingCommand(String),
    #[error("Error occurred while decoding")]
    Decoding(Bytes),
    #[error("Error occurred while encoding")]
    Encoding(Bytes),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
    #[error("Handler error: {0}")]
    HandlerError(HandlerError),
}
