use std::collections::VecDeque;
use std::sync::Arc;

use tauri::{Emitter, Manager, WindowEvent};
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

pub mod commands;
pub mod config;
pub mod dict_utils;
pub mod ime;
pub mod logger;
pub mod server;
pub mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Windows console attach when running from terminal
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            let _ = AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    let (app_logger_layer, mut log_rx) = logger::AppLoggerLayer::new();

    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(app_logger_layer)
        .with(fmt_layer)
        .init();

    let hide_window = std::env::args().any(|arg| arg == "hide");

    let log_history = Arc::new(Mutex::new(VecDeque::with_capacity(
        commands::LOG_HISTORY_CAPACITY,
    )));

    let log_history_clone = log_history.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["hide"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .setup(move |app| {
            let config = tauri::async_runtime::block_on(async {
                config::load_config().await.unwrap_or_default()
            });

            let server_ctrl = server::start(server::ServerState {
                config,
                running: true,
            });

            let app_handle = app.handle().clone();
            let mut state_rx = server_ctrl.subscribe();
            tauri::async_runtime::spawn(async move {
                while state_rx.changed().await.is_ok() {
                    let current_state = state_rx.borrow().clone();
                    let _ = app_handle.emit("server-state-changed", &current_state);
                }
            });

            let app_handle_for_log = app.handle().clone();
            let history_for_log = log_history_clone.clone();
            tauri::async_runtime::spawn(async move {
                let mut id_counter = 0u64;
                while let Ok(entry) = log_rx.recv().await {
                    id_counter += 1;
                    let formatted = entry.to_formatted(id_counter);
                    {
                        let mut hist = history_for_log.lock().await;
                        hist.push_back(formatted.clone());
                        if hist.len() > commands::LOG_HISTORY_CAPACITY {
                            hist.pop_front();
                        }
                    }
                    let _ = app_handle_for_log.emit("log-entry", &formatted);
                }
            });

            app.manage(commands::AppState {
                server_ctrl,
                log_history: log_history_clone,
            });

            // Setup system tray
            if let Err(e) = tray::setup_tray(app.handle()) {
                tracing::error!("Failed to setup tray: {:?}", e);
            }

            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            if let Some(window) = app.get_webview_window("main") {
                if hide_window {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_server_state,
            commands::set_server_running,
            commands::save_config,
            commands::refresh_dict_cache,
            commands::get_config_path,
            commands::get_recent_logs,
            commands::exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
