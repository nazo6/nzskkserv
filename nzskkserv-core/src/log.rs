use once_cell::sync::OnceCell;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

use crate::server::interface::{SkkIncomingEvent, SkkOutcomingEvent};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct LogEntry {
    pub event: LogEvent,
    pub level: u8,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum LogEvent {
    Incoming(SkkIncomingEvent),
    Outcoming(SkkOutcomingEvent),
    Message(String),
}

pub trait Logger: Sync + Send {
    fn log(&self, log: LogEntry);
}

struct NopLogger {}
impl Logger for NopLogger {
    fn log(&self, log: LogEntry) {}
}

static LOGGER: OnceCell<Box<dyn Logger>> = OnceCell::new();

pub fn set_logger(logger: impl Logger + 'static) {
    LOGGER.set(Box::new(logger));
}

pub(crate) fn log(log_entry: LogEntry) {
    let logger = LOGGER.get_or_init(|| Box::new(NopLogger {}));
    logger.log(log_entry)
}
