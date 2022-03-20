use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid incoming command {0}")]
    Server(nzskkserv_server::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Error {0}")]
    Other(String),
}
