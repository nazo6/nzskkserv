use bounded_vec_deque::BoundedVecDeque;
use dioxus::prelude::*;
use tracing::error;

use crate::logger::{LogData, LogEntry};

use super::LogReceiverContext;

fn use_log() -> ReadOnlySignal<BoundedVecDeque<LogEntry>> {
    let mut log_store = use_signal(|| BoundedVecDeque::<LogEntry>::new(128));
    let log_receiver: LogReceiverContext = use_context();

    use_future(move || {
        let log_receiver = log_receiver.clone();
        async move {
            loop {
                let mut log_rx = log_receiver.lock().await;
                match log_rx.recv().await {
                    Ok(entry) => {
                        log_store.write().push_back(entry);
                    }
                    Err(_) => {
                        error!("Log receiver channel closed, stopping log updates.");
                        break;
                    }
                }
            }
        }
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
