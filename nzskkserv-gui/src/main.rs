#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tokio::sync::watch;

mod app;
mod config;
mod server;
mod tray;

#[derive(PartialEq, Clone, Copy, Debug)]
enum AppState {
    Show,
    Hide,
    Quit,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("nzskkserv_gui=info,nzskkserv_core=info"),
    )
    .init();

    let config = config::load_config().await?;

    let server_controller = server::start(config);

    let (app_state_sender, app_state_receiver) = watch::channel(AppState::Show);
    tray::start(app_state_sender);

    app::start(app_state_receiver, server_controller);

    Ok(())
}
