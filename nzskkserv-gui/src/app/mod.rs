use dioxus::{
    desktop::{use_window, WindowBuilder},
    prelude::*,
};

#[cfg(not(debug_assertions))]
use directories::ProjectDirs;

use crate::{logger::LogReceiver, server::ServerStateController};

mod config;
mod log;
mod server_state;
mod start_stop_btn;
mod tray;

// NOTE: About Asset Management
// Since dioxus does not embed assets into binaries, I decided to use dioxus' asset management system only for debug builds,
// and embed css files directly into html for release builds.

#[cfg(debug_assertions)]
const MAIN_CSS: Asset = asset!("/assets/main.css");
#[cfg(debug_assertions)]
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

struct LogReceiverContext(pub LogReceiver);
impl Clone for LogReceiverContext {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

pub(super) fn start(server_ctrl: ServerStateController, log_rx: LogReceiver, hide_window: bool) {
    let vdom = VirtualDom::new(App)
        .with_root_context(server_ctrl)
        .with_root_context(LogReceiverContext(log_rx));

    let window = WindowBuilder::new()
        .with_visible(!hide_window)
        .with_title("nzskkserv");

    let config = dioxus::desktop::Config::new()
        .with_window(window)
        .with_menu(None)
        .with_icon(
            dioxus::desktop::tao::window::Icon::from_rgba(
                crate::icon::ICON_DATA.into(),
                crate::icon::ICON_WIDTH,
                crate::icon::ICON_HEIGHT,
            )
            .unwrap(),
        )
        .with_custom_event_handler(|_event, event_loop_target| {
            // Set activation policy to Accessory on macOS to prevent dock icon
            #[cfg(target_os = "macos")]
            {
                static POLICY_SET: std::sync::Once = std::sync::Once::new();
                POLICY_SET.call_once(|| {
                    use dioxus::desktop::tao::platform::macos::{
                        ActivationPolicy, EventLoopWindowTargetExtMacOS as _,
                    };

                    event_loop_target.set_activation_policy_at_runtime(ActivationPolicy::Accessory);
                });
            }
        })
        .with_close_behaviour(dioxus::desktop::WindowCloseBehaviour::LastWindowHides);

    #[cfg(not(debug_assertions))]
    let config = {
        let project_dirs =
            ProjectDirs::from("", "", "nzskkserv").expect("Could not found project dirs");
        let data_dir = project_dirs.data_dir().to_path_buf();
        let _ = std::fs::create_dir_all(&data_dir);

        config
            .with_custom_index(format!(
                r#"
<!DOCTYPE html>
<html>
  <head>
    <title>nzskkserv</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <style>{}</style>
    <style>{}</style>
  </head>
  <body>
    <div id="main"></div>
  </body>
</html>
        "#,
                include_str!("../../assets/tailwind.css"),
                include_str!("../../assets/main.css")
            ))
            .with_data_directory(data_dir)
    };

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
    let window = use_window();
    tray::use_tray_menu();

    rsx! {
        {
            #[cfg(debug_assertions)]
            rsx! {
                document::Link { rel: "stylesheet", href: MAIN_CSS }
                document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            }
        }
        div { class: "flex flex-col h-full",
            div { class: "tabs tabs-lift w-full",
                a {
                    class: "tab",
                    class: if *tab.read() == HomeTabItem::Log { "tab-active" },
                    onclick: move |_| *tab.write() = HomeTabItem::Log,
                    "Log"
                }
                a {
                    class: "tab mr-auto",
                    class: if *tab.read() == HomeTabItem::Config { "tab-active" },
                    onclick: move |_| *tab.write() = HomeTabItem::Config,
                    "Config"
                }
                start_stop_btn::ServerStartStop {}
                button {
                    class: "btn mr-2",
                    onclick: move |_| {
                        window.close();
                    },
                    "Quit"
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
