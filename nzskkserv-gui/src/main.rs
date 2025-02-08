use dioxus::{
    desktop::trayicon::{default_tray_icon, init_tray_icon},
    prelude::*,
};

mod config;
mod server;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    init_tray_icon(default_tray_icon(), None);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "flex flex-col h-full",
            div { class: "tabs tabs-lift w-full",
                a { class: "tab", "Log" }
                a { class: "tab", "Config" }
            }
            div { class: "h-full" }
        }
    }
}
