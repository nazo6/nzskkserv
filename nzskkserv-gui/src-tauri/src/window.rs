use std::sync::Mutex;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, WebviewWindowBuilder};

#[derive(Default)]
pub struct WindowState {
    last_position: Mutex<Option<LogicalPosition<f64>>>,
    last_size: Mutex<Option<LogicalSize<f64>>>,
}

/// Opens the main WebviewWindow.
/// If the window already exists, it is shown, unminimized, and focused.
/// If it does not exist, it is created using the configuration from `tauri.conf.json`,
/// with previous position and size restored if available.
pub fn open_main_window(app: &AppHandle) {
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
            return;
        }

        if let Some(config) = app_handle.config().app.windows.iter().find(|w| w.label == "main") {
            match WebviewWindowBuilder::from_config(&app_handle, config) {
                Ok(mut builder) => {
                    if let Some(state) = app_handle.try_state::<WindowState>() {
                        if let Some(pos) = *state.last_position.lock().unwrap() {
                            builder = builder.position(pos.x, pos.y);
                        }
                        if let Some(size) = *state.last_size.lock().unwrap() {
                            builder = builder.inner_size(size.width, size.height);
                        }
                    }

                    match builder.build() {
                        Ok(window) => {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                        Err(e) => {
                            tracing::error!("Failed to build main window: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to create window builder from config: {:?}", e);
                }
            }
        } else {
            tracing::error!("Main window configuration not found in tauri.conf.json");
        }
    });
}

/// Saves the current window position and size, then destroys the main WebviewWindow.
/// Destroying the window frees the Webview process and runtime resources.
pub fn destroy_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if let Some(state) = app.try_state::<WindowState>() {
            if let Ok(scale_factor) = window.scale_factor() {
                if let Ok(pos) = window.outer_position() {
                    let logical_pos = pos.to_logical::<f64>(scale_factor);
                    *state.last_position.lock().unwrap() = Some(logical_pos);
                }
                if let Ok(size) = window.inner_size() {
                    let logical_size = size.to_logical::<f64>(scale_factor);
                    *state.last_size.lock().unwrap() = Some(logical_size);
                }
            }
        }
        let _ = window.destroy();
    }
}

/// Toggles the main window:
/// - If the window exists and is visible, it is destroyed to free memory.
/// - Otherwise, the window is created/shown.
pub fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(true) = window.is_visible() {
            destroy_main_window(app);
            return;
        }
    }
    open_main_window(app);
}

