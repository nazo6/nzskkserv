pub mod manager;
pub mod config;
pub mod error;
pub use error::Error;
mod dict_utils;

pub use config::Dict;
use nzskkserv_server::Dict as DictData;
