use dioxus::{
    desktop::trayicon::{default_tray_icon, init_tray_icon},
    prelude::*,
};

use crate::server::ServerStateController;

mod server_state;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub(super) fn start(server_ctrl: ServerStateController) {
    let vdom = VirtualDom::new(App).with_root_context(server_ctrl);
    let config = dioxus::desktop::Config::new();

    dioxus::desktop::launch::launch_virtual_dom_blocking(vdom, config);
}

#[component]
fn App() -> Element {
    init_tray_icon(default_tray_icon(), None);

    let server_state = server_state::use_server_state();
    let set_server_state = server_state::use_set_server_state();

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "flex flex-col h-full",
            div { class: "tabs tabs-lift w-full",
                a { class: "tab", "Log" }
                a { class: "tab", "Config" }
            }
            div { class: "h-full",
                div { "{server_state.read().running}" }
                button {
                    onclick: {
                        move |_| {
                            set_server_state
                                .send_modify(|state| {
                                    state.running = !state.running;
                                })
                        }
                    },
                    "Toggle"
                }
            }
        }
    }
}
