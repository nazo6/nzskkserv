use anyhow::Result;
use config::load_config;
use crossbeam::channel::unbounded;
use dict_utils::load_dicts;
use log::{ServerLogger, LOGGER};
use nzskkserv_core::server::ServerConfig;
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
    thread,
};
use tray_item::TrayItem;

mod app;
mod config;
mod dict_utils;
mod log;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

enum Message {
    Quit,
    ShowHide,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logger
    let server_logger = ServerLogger {
        global_logger: &LOGGER,
    };
    nzskkserv_core::log::set_logger(server_logger)?;

    // Setup task tray
    let mut tray = TrayItem::new("nzskkserv", "tray-icon").unwrap();
    let is_hidden = Arc::new(Mutex::new(true));

    let (s, r) = unbounded();

    let s1 = s.clone();
    tray.add_menu_item("Show/Hide", move || {
        s1.send(Message::ShowHide).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Quit", move || {
        s.send(Message::Quit).unwrap();
    })
    .unwrap();

    // Load config and dicts
    let config = load_config().await;
    let encoding = match config.server_encoding {
        config::Encoding::Utf8 => nzskkserv_core::Encoding::Utf8,
        config::Encoding::Eucjp => nzskkserv_core::Encoding::Eucjp,
    };
    let dicts = load_dicts(config.dicts).await;
    let server_config = ServerConfig {
        dicts,
        enable_google_cgi: config.enable_google_cgi,
        encoding,
    };

    // Setup server
    let server = nzskkserv_core::Server::new(LOCALHOST, config.port.unwrap_or(1178), server_config);
    let server = Arc::new(server);

    // Setup window
    let app = app::App::new(Arc::clone(&is_hidden), Arc::clone(&server));
    let options = eframe::NativeOptions::default();
    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(Message::Quit) => break,
            Ok(Message::ShowHide) => {
                let mut val = is_hidden.lock().unwrap();
                dbg!(&val);
                *val = !*val;
            }
            _ => {}
        }
    });

    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(app)));

    handle.join().unwrap();

    Ok(())
}
