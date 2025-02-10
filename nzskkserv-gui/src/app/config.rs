use dioxus::prelude::*;
use tracing::info;

use crate::{app::server_state, config::Encoding};

mod dict_editor;

#[component]
pub(super) fn ConfigPanel() -> Element {
    let server_state = server_state::use_server_state();
    let set_server_state = server_state::use_set_server_state();

    let mut modified_config = use_signal(|| server_state.read().config.clone());

    let auto_launch = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("nzskkserv")
        .set_app_path(std::env::current_exe().unwrap().to_str().unwrap())
        .set_args(&["hide"])
        .build()
        .unwrap();
    let mut auto_launch_enabled = use_signal(|| auto_launch.is_enabled().unwrap());

    rsx! {
        div { class: "flex justify-center",
            div { class: "h-full p-1 flex flex-col gap-3 w-[50rem]",
                div {
                    p { "Config path: {crate::config::CONFIG_PATH.to_string_lossy()}" }
                    p { class: "font-bold text-2xl", "Auto start" }
                    input {
                        r#type: "checkbox",
                        class: "col-span-3 checkbox",
                        checked: auto_launch_enabled,
                        onchange: move |ev| {
                            if ev.checked() {
                                if let Err(e) = auto_launch.enable() {
                                    tracing::error!("Failed to enable auto launch: {:?}", e);
                                    return;
                                }
                            } else if let Err(e) = auto_launch.disable() {
                                tracing::error!("Failed to disable auto launch: {:?}", e);
                                return;
                            }
                            info!("Successfully changed auto launch state: {}", ev.checked());
                            auto_launch_enabled.set(ev.checked());
                        },
                    }
                }

                p { class: "font-bold text-2xl", "Server Settings" }

                p { class: "font-bold text-lg", "Dictonaries" }
                dict_editor::DictsEditor {
                    dicts: modified_config.read().dicts.clone(),
                    onchange: move |dicts| {
                        modified_config.write().dicts = dicts;
                    },
                }

                p { class: "font-bold text-lg", "Config" }
                div { class: "grid grid-cols-5 gap-y-2",
                    div { class: "col-span-2", "Port" }
                    input {
                        r#type: "number",
                        class: "col-span-3 input w-full",
                        value: modified_config.read().port.to_string(),
                        oninput: move |ev| {
                            if let Ok(port) = ev.value().parse() {
                                modified_config.write().port = port;
                            }
                        },
                    }

                    div { class: "col-span-2", "Server encoding" }
                    div { class: "col-span-3",
                        EncodingSelector {
                            encoding: modified_config.read().server_encoding.clone(),
                            onchange: move |encoding| {
                                modified_config.write().server_encoding = encoding;
                            },
                        }
                    }

                    div { class: "col-span-2", "Enable google cgi" }
                    input {
                        r#type: "checkbox",
                        class: "col-span-3 checkbox",
                        checked: modified_config.read().enable_google_cgi,
                        onchange: move |ev| {
                            modified_config.write().enable_google_cgi = ev.checked();
                        },
                    }
                }


                div { class: "divider" }
                div { class: "ml-auto flex gap-2",
                    button {
                        class: "btn",
                        onclick: {
                            move |_| {
                                *modified_config.write() = server_state.read().config.clone();
                            }
                        },
                        "Reset"
                    }
                    button {
                        class: "btn",
                        onclick: {
                            move |_| {
                                set_server_state
                                    .send_modify(|state| {
                                        state.config = modified_config.read().clone();
                                    })
                            }
                        },
                        "Apply"
                    }
                }
            }
        }
    }
}

impl Encoding {
    fn to_str(&self) -> String {
        match self {
            Encoding::Utf8 => "UTF-8".to_string(),
            Encoding::Eucjp => "EUC-JP".to_string(),
        }
    }
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "UTF-8" => Some(Encoding::Utf8),
            "EUC-JP" => Some(Encoding::Eucjp),
            _ => None,
        }
    }
}

#[component]
fn EncodingSelector(encoding: Encoding, onchange: Callback<Encoding>) -> Element {
    rsx! {
        select {
            class: "select w-full",
            value: encoding.to_str(),
            onchange: move |ev| {
                if let Some(new_encoding) = Encoding::from_str(ev.data.value().as_str()) {
                    onchange.call(new_encoding);
                }
            },
            option { "UTF-8" }
            option { "EUC-JP" }
        }
    }
}
