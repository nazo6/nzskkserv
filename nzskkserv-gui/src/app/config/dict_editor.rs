use std::path::PathBuf;

use dioxus::prelude::*;
use tracing::{info, warn};

use super::EncodingSelector;
use crate::{
    config::Encoding,
    dict_utils::{DictDef, DictFormat, DictPath},
};

#[component]
pub(super) fn DictsEditor(dicts: Vec<DictDef>, onchange: Callback<Vec<DictDef>>) -> Element {
    rsx! {
        div {
            table { class: "table table-xm",
                thead {
                    tr {
                        th { "Source type" }
                        th { "URL/Path" }
                        th { "Encoding" }
                        th { "Format" }
                        th {}
                    }
                }
                tbody {
                    for (i , dict) in dicts.iter().enumerate() {
                        tr {
                            DictRow {
                                dict: dict.clone(),
                                onchange: {
                                    let dicts = dicts.clone();
                                    move |new_dict| {
                                        let mut dicts = dicts.clone();
                                        if let Some(new_dict) = new_dict {
                                            dicts[i] = new_dict;
                                        } else {
                                            dicts.remove(i);
                                        }
                                        onchange.call(dicts);
                                    }
                                },
                            }
                        }
                    }
                }
            }
            div {
                button {
                    class: "btn",
                    onclick: {
                        let dicts = dicts.clone();
                        move |_| {
                            let mut dicts = dicts.clone();
                            dicts
                                .push(DictDef {
                                    path_or_url: DictPath::File {
                                        path: PathBuf::new(),
                                    },
                                    encoding: Encoding::Utf8,
                                    format: DictFormat::Skk,
                                });
                            onchange.call(dicts);
                        }
                    },
                    "Add"
                }
            }
        }
    }
}

#[component]
fn DictRow(dict: DictDef, onchange: Callback<Option<DictDef>>) -> Element {
    rsx! {
        td {
            select {
                class: "select",
                value: dict.to_type_str(),
                onchange: {
                    let dict = dict.clone();
                    move |ev: Event<FormData>| {
                        let mut new_dict = dict.clone();
                        if let Err(e) = new_dict.set_type(ev.value().as_str()) {
                            tracing::error!("Failed to parse type: {:?}", e);
                        }
                        onchange.call(Some(new_dict));
                    }
                },
                option { value: "File", "File" }
                option { value: "Url", "URL" }
            }
        }
        td {
            input {
                class: "input",
                value: dict.get_path_url_str(),
                onchange: {
                    let dict = dict.clone();
                    move |ev: Event<FormData>| {
                        let mut new_dict = dict.clone();
                        if let Err(e) = new_dict.set_path_url(ev.value().as_str()) {
                            tracing::error!("Failed to parse URL: {:?}", e);
                        }
                        onchange.call(Some(new_dict));
                    }
                },
            }
        }
        td {
            EncodingSelector {
                encoding: dict.encoding.clone(),
                onchange: {
                    let dict = dict.clone();
                    move |new_encoding| {
                        let mut new_dict = dict.clone();
                        new_dict.encoding = new_encoding;
                        onchange.call(Some(new_dict));
                    }
                },
            }
        }
        td {
            select {
                class: "select",
                value: dict.format.to_str(),
                onchange: {
                    let dict = dict.clone();
                    move |ev: Event<FormData>| {
                        let mut new_dict = dict.clone();
                        new_dict.format = DictFormat::from_str(ev.value().as_str());
                        onchange.call(Some(new_dict));
                    }
                },
                option { value: "Skk", "SKK" }
                option { value: "Mozc", "mozc" }
            }
        }
        td {
            button {
                class: "btn btn-square",
                disabled: !matches!(dict.path_or_url, DictPath::Url { .. }),
                onclick: {
                    let path = dict.path_or_url.clone();
                    move |_| {
                        let path = path.clone();
                        spawn_forever(async move {
                            if let DictPath::Url { url } = path {
                                match url.cache_and_get(true).await {
                                    Ok(_) => {
                                        info!("Dictionary cache updated: {}", url.0);
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to update dictionary cache: {}, error: {}", url.0, e
                                        );
                                    }
                                }
                            }
                        });
                    }
                },
                "R"
            }
        }
        td {
            button {
                class: "btn btn-square",
                onclick: {
                    move |_| {
                        onchange.call(None);
                    }
                },
                "X"
            }
        }
    }
}
