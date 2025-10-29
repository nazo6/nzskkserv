#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

mod app;
mod config;
mod dict_utils;
mod logger;
mod server;

mod icon {
    include!(concat!(env!("OUT_DIR"), "/icon.rs"));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // With this code, if program is launched from console, program can output to console.
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        unsafe {
            let _ = AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    let (app_logger_layer, log_rx) = logger::AppLoggerLayer::new();

    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(app_logger_layer)
        .with(fmt_layer)
        .init();

    let config = config::load_config().await?;

    let server_ctrl = server::start(server::ServerState {
        config,
        running: true,
    });

    app::start(
        server_ctrl,
        log_rx,
        std::env::args().any(|arg| arg == "hide"),
    );

    Ok(())
}
