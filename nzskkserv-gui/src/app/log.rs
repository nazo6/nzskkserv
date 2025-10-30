use std::collections::VecDeque;

use dioxus::prelude::*;
use tracing::error;

use crate::logger::{LogData, LogEntry};

use super::LogReceiverContext;

fn use_log() -> ReadOnlySignal<VecDeque<LogEntry>> {
    let mut log_store = use_signal(|| VecDeque::<LogEntry>::with_capacity(8));
    let mut log_receiver: LogReceiverContext = use_context();

    use_hook(move || {
        spawn(async move {
            dbg!("spawn");
            loop {
                match log_receiver.0.recv().await {
                    Ok(entry) => {
                        let mut log_store = log_store.write();
                        dbg!(log_store.len());
                        if log_store.len() == log_store.capacity() {
                            log_store.pop_front();
                        }
                        log_store.push_back(entry);
                    }
                    Err(_) => {
                        error!("Log receiver channel closed, stopping log updates.");
                        break;
                    }
                }
            }
        });
    });

    ReadOnlySignal::new(log_store)
}

#[component]
pub(super) fn LogPanel() -> Element {
    let log = use_log();

    rsx! {
        div { class: "h-full p-1",
            table { class: "table table-sm",
                thead {
                    tr {
                        th { "Time" }
                        th { "Level" }
                        th { "Target" }
                        th { "Message" }
                    }
                }
                tbody {
                    for entry in log.read().iter().rev() {
                        {
                            let time = entry.time.strftime("%F %T");
                            let mes = format_log_data(entry.data.clone());
                            rsx! {
                                tr {
                                    td { "{time}" }
                                    td { "{entry.level}" }
                                    td { "{entry.target}" }
                                    td { "{mes}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_log_data(data: LogData) -> String {
    match data {
        LogData::Message(m) => m,
        LogData::ConvertInput(i) => i,
        LogData::ConvertOutput(o) => o,
    }
}
