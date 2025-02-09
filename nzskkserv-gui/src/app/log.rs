use bounded_vec_deque::BoundedVecDeque;
use dioxus::prelude::*;
use jiff::Unit;

use crate::logger::LogEntry;

use super::LogReceiverContext;

// NOTE: It is not expected to call this hook multiple times.
// If multiple instance of this hook exists, app will be blocked.
#[allow(
    clippy::await_holding_lock,
    reason = "If this hook is called only once, mutex actually never blocks."
)]
fn use_log() -> ReadOnlySignal<BoundedVecDeque<LogEntry>> {
    let mut log_store = use_signal(|| BoundedVecDeque::<LogEntry>::new(128));
    let log_receiver: LogReceiverContext = use_context();

    use_effect(move || {
        let log_receiver = log_receiver.clone();
        spawn(async move {
            loop {
                let mut log_rx = log_receiver.lock().unwrap();
                if let Ok(entry) = log_rx.recv().await {
                    log_store.write().push_back(entry);
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
            div { "Log" }
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
                        match entry {
                            LogEntry::Tracing { time, level, target, name, message } => {
                                let time = time.strftime("%F %T");
                                rsx! {
                                    tr {
                                        td { "{time}" }
                                        td { "{level}" }
                                        td { "{target}" }
                                        td { "{message}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
