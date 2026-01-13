use std::collections::VecDeque;

use dioxus::prelude::*;
use tracing::error;

use crate::logger::{LogData, LogEntry};

use super::LogReceiverContext;

fn use_log() -> ReadSignal<VecDeque<LogEntry>, SyncStorage> {
    let mut log_store = use_signal_sync(|| VecDeque::<LogEntry>::with_capacity(128));
    let mut log_receiver: LogReceiverContext = use_context();

    use_hook(move || {
        // NOTE: dioxusのspawnを使うと、126会程度logの受け取りが行われた後にハングする。謎。
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                match log_receiver.0.recv().await {
                    Ok(entry) => {
                        let mut log_store = log_store.write();
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

    ReadSignal::new_maybe_sync(log_store)
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
