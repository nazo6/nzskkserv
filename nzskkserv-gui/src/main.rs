mod app;
mod config;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let config = config::load_config().await?;

    let server_ctrl = server::start(server::ServerState {
        config,
        running: true,
    });

    app::start(server_ctrl);

    Ok(())
}
