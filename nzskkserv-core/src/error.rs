use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid incoming command {0}")]
    Server(nzskkserv_server::Error),
    #[error("Error occurred while loading config: {0}")]
    ConfigRead(String),
    #[error("Error occurred while writing config: {0}")]
    ConfigWrite(String),
    #[error("Error occurred while writing dictionary: {0}")]
    DictRead(String),
    #[error("Error occurred while writing dictionary: {0}")]
    DictWrite(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Error {0}")]
    Other(String),
}
