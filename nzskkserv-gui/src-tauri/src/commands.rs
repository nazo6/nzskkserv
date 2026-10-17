use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use url::Url;

use crate::{
    config::{self, Config, CONFIG_PATH},
    dict_utils::DictUrl,
    logger::FormattedLogEntry,
    server::{ServerState, ServerStateController},
};

pub struct AppState {
    pub server_ctrl: ServerStateController,
    pub log_history: Arc<Mutex<bounded_vec_deque::BoundedVecDeque<FormattedLogEntry>>>,
}

#[tauri::command]
pub async fn get_server_state(state: State<'_, AppState>) -> Result<ServerState, String> {
    Ok(state.server_ctrl.borrow().clone())
}

#[tauri::command]
pub async fn set_server_running(
    running: bool,
    state: State<'_, AppState>,
) -> Result<ServerState, String> {
    state
        .server_ctrl
        .send_modify(|s| {
            s.running = running;
        });
    Ok(state.server_ctrl.borrow().clone())
}

#[tauri::command]
pub async fn save_config(
    new_config: Config,
    state: State<'_, AppState>,
) -> Result<ServerState, String> {
    config::write_config(&new_config)
        .await
        .map_err(|e| format!("Failed to write config: {}", e))?;

    state
        .server_ctrl
        .send_modify(|s| {
            s.config = new_config;
        });

    Ok(state.server_ctrl.borrow().clone())
}

#[tauri::command]
pub async fn refresh_dict_cache(url_str: String) -> Result<(), String> {
    let parsed_url = Url::parse(&url_str).map_err(|e| format!("Invalid URL: {}", e))?;
    let dict_url = DictUrl(parsed_url);
    dict_url
        .cache_and_get(true)
        .await
        .map_err(|e| format!("Failed to cache dictionary: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_config_path() -> String {
    CONFIG_PATH.to_string_lossy().to_string()
}

#[tauri::command]
pub async fn get_recent_logs(state: State<'_, AppState>) -> Result<Vec<FormattedLogEntry>, String> {
    let history = state.log_history.lock().await;
    Ok(history.iter().cloned().collect())
}

#[tauri::command]
pub fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}


