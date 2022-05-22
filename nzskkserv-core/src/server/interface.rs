#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum SkkIncomingEvent {
    /// 0
    Disconnect,
    /// 1
    Convert(String),
    /// 2
    Version,
    /// 3
    Hostname,
    /// 4
    Server,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum SkkOutcomingEvent {
    Convert(Option<String>),
    Version,
    Hostname,
    Server,
}
