use std::net::{IpAddr, Ipv4Addr};

use flutter_rust_bridge::StreamSink;
use nzskkserv_core::{
    log::{LogEntry, Logger},
    Server,
};
use once_cell::sync::OnceCell;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

pub fn greet() -> String {
    "Hello from Rust! 🦀".into()
}

struct FlutterLogger {
    sink: StreamSink<String>,
}

impl Logger for FlutterLogger {
    fn log(&self, log: LogEntry) {
        self.sink.add(serde_json::to_string(&log).unwrap());
    }
}

static SERVER: OnceCell<Server> = OnceCell::new();

#[tokio::main]
pub async fn start(sink: StreamSink<String>) -> Result<(), String> {
    if SERVER.get().is_some() {
        return Err("Server already started".to_string());
    }
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    nzskkserv_core::log::set_logger(FlutterLogger { sink });

    let encoding = nzskkserv_core::Encoding::Utf8;

    let server = nzskkserv_core::Server::new(LOCALHOST, 1178, Vec::new(), true, encoding);

    SERVER.set(server);

    let start_server = || async {
        let _ = SERVER.get().unwrap().start().await;
    };

    tokio::join! {
        start_server(),
    };

    Ok(())
}

#[tokio::main]
pub async fn stop() -> Result<(), String> {
    let server = SERVER
        .get()
        .ok_or_else(|| "Server not started".to_string())?;

    server
        .stop()
        .map(|_| ())
        .map_err(|_| "Errour occurred while stopping server".to_string())
}
