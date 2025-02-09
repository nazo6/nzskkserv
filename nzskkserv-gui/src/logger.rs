use std::fmt;
use std::fmt::Write as _;

use tokio::sync::broadcast;
use tracing::field::{Field, Visit};
use tracing_subscriber::Layer;

pub(crate) struct AppLoggerLayer {
    sender: broadcast::Sender<LogEntry>,
}

pub type LogReceiver = broadcast::Receiver<LogEntry>;

#[derive(Clone, Debug)]
pub(crate) enum LogEntry {
    Tracing {
        time: jiff::Zoned,
        level: tracing::Level,
        target: String,
        name: String,
        message: String,
    },
}

impl AppLoggerLayer {
    pub fn new() -> (Self, LogReceiver) {
        let (sender, receiver) = broadcast::channel(100);
        (Self { sender }, receiver)
    }
}

pub struct StringVisitor<'a> {
    string: &'a mut String,
}

impl Visit for StringVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            write!(self.string, "{:?}", value).unwrap();
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
        let mut message = String::new();
        event.record(&mut StringVisitor {
            string: &mut message,
        });

        let _ = self.sender.send(LogEntry::Tracing {
            time: jiff::Zoned::now(),
            level: event.metadata().level().to_owned(),
            target: event.metadata().target().to_owned(),
            name: event.metadata().name().to_owned(),
            message,
        });
    }
}
