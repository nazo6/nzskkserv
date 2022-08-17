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
pub(crate) enum GlobalLogEntry {
    ServerLog(LogEntry),
    AppLog(String),
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
    pub(crate) fn log(&self, entry: String) {
        self.logs
            .lock()
            .unwrap()
            .push(GlobalLogEntry::AppLog(entry));
    }
    pub(crate) fn get_logs(&self) -> Vec<GlobalLogEntry> {
        (*self.logs.lock().unwrap()).clone()
    }
}

pub(crate) static LOGGER: Lazy<GlobalLogger> = Lazy::new(|| GlobalLogger {
    logs: Arc::new(Mutex::new(Vec::new())),
});
