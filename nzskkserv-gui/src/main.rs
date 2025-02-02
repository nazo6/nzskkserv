#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

mod app;
mod config;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config().await?;

    server::create_server(config).await;

    app::start();

    Ok(())
}
