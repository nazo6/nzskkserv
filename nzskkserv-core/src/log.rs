use once_cell::sync::OnceCell;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

use crate::{
    server::interface::{SkkIncomingEvent, SkkOutGoingEvent},
    Error,
};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub event: LogEvent,
    pub level: u8,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum LogEvent {
    Incoming(SkkIncomingEvent),
    OutGoing(SkkOutGoingEvent),
    Message(String),
}

pub trait Logger: Sync + Send {
    fn log(&self, log: LogEntry);
}

struct NopLogger {}
impl Logger for NopLogger {
    fn log(&self, _log: LogEntry) {}
}

static LOGGER: OnceCell<Box<dyn Logger>> = OnceCell::new();

pub fn set_logger(logger: impl Logger + 'static) -> Result<(), Error> {
    LOGGER.set(Box::new(logger)).map_err(|_| Error::LoggerSet)
}

pub(crate) fn log(log_entry: LogEntry) {
    let logger = LOGGER.get_or_init(|| Box::new(NopLogger {}));
    logger.log(log_entry)
}

#[macro_export]
macro_rules! log_msg {
    ($lvl:expr, $arg:expr) => {
        $crate::log::log($crate::log::LogEntry {
            event: $crate::log::LogEvent::Message($arg.to_string()),
            level: $lvl,
        })
    };
    ($lvl:expr, $arg:expr, $( $format_args:expr ),*) => {
        $crate::log::log($crate::log::LogEntry {
            event: $crate::log::LogEvent::Message(format!($arg, $( $format_args ),* )),
            level: $lvl,
        })
    };
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    ($( $args:expr ),*) => {
        log_msg!(0, $( $args ),*)
    };
}
#[macro_export(local_inner_macros)]
macro_rules! warn {
    ($( $args:expr ),*) => {
        log_msg!(0, $( $args ),*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    ($( $args:expr ),*) => {
        log_msg!(0, $( $args ),*)
    };
}
