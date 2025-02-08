use dioxus::prelude::*;

use crate::app::server_state;

#[component]
pub(super) fn ConfigPanel() -> Element {
    let server_state = server_state::use_server_state();
    let set_server_state = server_state::use_set_server_state();

    rsx! {
        div { class: "h-full p-1 flex flex-col",
            button {
                class: "btn",
                onclick: {
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
            div {
                table { class: "table table-xm",
                    thead {
                        tr {
                            th { "Type" }
                            th { "URL/Path" }
                            th { "Encoding" }
                        }
                    }
                    tbody {
                        for dict in &server_state.read().config.dicts {
                            tr {
                                td { "" }
                                td { "" }
                                td { "{dict.encoding:?}" }
                            }
                        }
                    }
                }

            }
        }
    }
}
