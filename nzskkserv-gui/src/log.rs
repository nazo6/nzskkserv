use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use nzskkserv_core::log::{LogEntry, Logger};

pub(crate) struct ServerLogger {
    pub(crate) global_logger: &'static GlobalLogger,
}

impl Logger for ServerLogger {
    fn log(&self, log: LogEntry) {
        self.global_logger.server_log(log);
    }
}

#[derive(Clone)]
pub(crate) struct AppLogEntry {
    pub level: u8,
    pub message: String,
}

#[derive(Clone)]
pub(crate) enum GlobalLogEntry {
    ServerLog(LogEntry),
    AppLog(AppLogEntry),
}

pub(crate) struct GlobalLogger {
    pub(crate) logs: Arc<Mutex<Vec<GlobalLogEntry>>>,
}

impl GlobalLogger {
    pub(crate) fn server_log(&self, entry: LogEntry) {
        self.logs
            .lock()
            .unwrap()
            .push(GlobalLogEntry::ServerLog(entry));
    }
    pub(crate) fn log(&self, level: u8, message: String) {
        self.logs
            .lock()
            .unwrap()
            .push(GlobalLogEntry::AppLog(AppLogEntry { level, message }));
    }
    pub(crate) fn get_logs(&self) -> Vec<GlobalLogEntry> {
        (*self.logs.lock().unwrap()).clone()
    }
}

pub(crate) static LOGGER: Lazy<GlobalLogger> = Lazy::new(|| GlobalLogger {
    logs: Arc::new(Mutex::new(Vec::new())),
});

pub(crate) fn log(level: u8, message: String) {
    LOGGER.log(level, message);
}

#[macro_export]
macro_rules! log_msg {
    ($lvl:expr, $arg:expr) => {
        $crate::log::log($lvl, $arg.to_string())
    };
    ($lvl:expr, $arg:expr, $( $format_args:expr ),*) => {
        $crate::log::log($lvl, format!($arg, $( $format_args ),* ))
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
