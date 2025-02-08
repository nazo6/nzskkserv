use dioxus::prelude::*;

#[component]
pub(super) fn LogPanel() -> Element {
    rsx! {
        div { class: "h-full", "Log" }
    }
}
