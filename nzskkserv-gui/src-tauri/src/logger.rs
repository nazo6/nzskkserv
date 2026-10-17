use std::fmt;

use tokio::sync::broadcast;
use tracing::field::{Field, Visit};
use tracing_subscriber::Layer;

use serde::{Deserialize, Serialize};

pub(crate) struct AppLoggerLayer {
    sender: broadcast::Sender<LogEntry>,
}

pub type LogReceiver = broadcast::Receiver<LogEntry>;

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub time: jiff::Zoned,
    pub level: tracing::Level,
    pub target: String,
    pub data: LogData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum LogData {
    Message(String),
    ConvertInput(String),
    ConvertOutput(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormattedLogEntry {
    pub id: u64,
    pub time: String,
    pub level: String,
    pub target: String,
    pub data: LogData,
}

impl LogEntry {
    pub fn to_formatted(&self, id: u64) -> FormattedLogEntry {
        FormattedLogEntry {
            id,
            time: self.time.strftime("%F %T").to_string(),
            level: self.level.as_str().to_string(),
            target: self.target.clone(),
            data: self.data.clone(),
        }
    }
}

impl AppLoggerLayer {
    pub fn new() -> (Self, LogReceiver) {
        let (sender, receiver) = broadcast::channel(100);
        (Self { sender }, receiver)
    }
}

pub struct LogVisitor {
    data: Option<LogData>,
}

impl Visit for LogVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.data = Some(LogData::Message(format!("{:?}", value)));
        } else if field.name() == "nzskkserv_input" {
            self.data = Some(LogData::ConvertInput(format!("{:?}", value)));
        } else if field.name() == "nzskkserv_output" {
            self.data = Some(LogData::ConvertOutput(format!("{:?}", value)));
        }
    }
}

impl<S> Layer<S> for AppLoggerLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = LogVisitor { data: None };
        event.record(&mut visitor);

        if let Some(data) = visitor.data {
            if event.metadata().target().starts_with("nzskkserv") {
                let _ = self.sender.send(LogEntry {
                    time: jiff::Zoned::now(),
                    level: event.metadata().level().to_owned(),
                    target: event.metadata().target().to_owned(),
                    // name: event.metadata().name().to_owned(),
                    data,
                });
            }
        }
    }
}
