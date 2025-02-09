use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

use crate::{logger::LogReceiver, server::ServerStateController};

mod config;
mod log;
mod server_state;
mod tray;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

type LogReceiverContext = Arc<Mutex<LogReceiver>>;

pub(super) fn start(server_ctrl: ServerStateController, log_rx: LogReceiver) {
    let vdom = VirtualDom::new(App)
        .with_root_context(server_ctrl)
        .with_root_context(Arc::new(Mutex::new(log_rx)));
    let config = dioxus::desktop::Config::new()
        .with_close_behaviour(dioxus::desktop::WindowCloseBehaviour::LastWindowHides);

    dioxus::desktop::launch::launch_virtual_dom_blocking(vdom, config);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HomeTabItem {
    Config,
    Log,
}

#[component]
fn App() -> Element {
    let mut tab = use_signal(|| HomeTabItem::Config);
    tray::use_tray_menu();

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "flex flex-col h-full",
            div { class: "tabs tabs-lift w-full",
                a {
                    class: "tab",
                    class: if *tab.read() == HomeTabItem::Log { "tab-active" },
                    onclick: move |_| *tab.write() = HomeTabItem::Log,
                    "Log"
                }
                a {
                    class: "tab",
                    class: if *tab.read() == HomeTabItem::Config { "tab-active" },
                    onclick: move |_| *tab.write() = HomeTabItem::Config,
                    "Config"
                }
            }
            div { class: "h-full overflow-auto",
                div {
                    class: "border border-base-300",
                    class: if *tab.read() != HomeTabItem::Log { "hidden" },
                    log::LogPanel {}
                }
                div {
                    class: "border border-base-300",
                    class: if *tab.read() != HomeTabItem::Config { "hidden" },
                    config::ConfigPanel {}
                }
            }
        }
    }
}
