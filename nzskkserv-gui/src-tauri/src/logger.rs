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
pub struct CandidateItem {
    pub candidate: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversionData {
    pub input: String,
    pub candidates: Vec<CandidateItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum LogData {
    Message(String),
    ConvertInput(String),
    ConvertOutput(String),
    Conversion(ConversionData),
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

#[derive(Default)]
pub struct LogVisitor {
    message: Option<String>,
    input: Option<String>,
    output: Option<String>,
    candidates: Option<String>,
}

impl LogVisitor {
    pub fn finish(self) -> Option<LogData> {
        if let Some(input) = self.input {
            if let Some(candidates_json) = self.candidates {
                if let Ok(candidates) = serde_json::from_str::<Vec<CandidateItem>>(&candidates_json) {
                    return Some(LogData::Conversion(ConversionData {
                        input,
                        candidates,
                    }));
                }
            }
            return Some(LogData::ConvertInput(input));
        }

        if let Some(output) = self.output {
            return Some(LogData::ConvertOutput(output));
        }

        if let Some(message) = self.message {
            return Some(LogData::Message(message));
        }

        None
    }
}

impl Visit for LogVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "message" => self.message = Some(value.to_string()),
            "input" | "nzskkserv_input" => self.input = Some(value.to_string()),
            "output" | "nzskkserv_output" => self.output = Some(value.to_string()),
            "candidates" => self.candidates = Some(value.to_string()),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        match field.name() {
            "message" => {
                if self.message.is_none() {
                    self.message = Some(format!("{:?}", value));
                }
            }
            "input" | "nzskkserv_input" => {
                if self.input.is_none() {
                    self.input = Some(format!("{:?}", value));
                }
            }
            "output" | "nzskkserv_output" => {
                if self.output.is_none() {
                    self.output = Some(format!("{:?}", value));
                }
            }
            "candidates" => {
                if self.candidates.is_none() {
                    self.candidates = Some(format!("{:?}", value));
                }
            }
            _ => {}
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
        let mut visitor = LogVisitor::default();
        event.record(&mut visitor);

        if let Some(data) = visitor.finish() {
            if event.metadata().target().starts_with("nzskkserv") {
                let _ = self.sender.send(LogEntry {
                    time: jiff::Zoned::now(),
                    level: event.metadata().level().to_owned(),
                    target: event.metadata().target().to_owned(),
                    data,
                });
            }
        }
    }
}
