use dioxus::prelude::*;

use crate::app::server_state;

#[component]
pub(super) fn ServerStartStop() -> Element {
    let server_state = server_state::use_server_state();
    let set_server_state = server_state::use_set_server_state();

    rsx! {
        div { class: "flex items-center",
            p { class: "font-bold",
                if server_state.read().running {
                    "Server is running"
                } else {
                    "Server is stopped"
                }
            }
            button {
                class: "btn mx-3",
                onclick: {
                    let set_server_state = set_server_state.clone();
                    move |_| {
                        set_server_state
                            .send_modify(|state| { state.running = !server_state.read().running })
                    }
                },
                if server_state.read().running {
                    "Stop"
                } else {
                    "Start"
                }
            }
        }
    }
}
