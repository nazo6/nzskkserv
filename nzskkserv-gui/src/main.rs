#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    info!("App started");
    // Setup logger
    let server_logger = ServerLogger {
        global_logger: &LOGGER,
    };
    nzskkserv_core::log::set_logger(server_logger)?;

    // Setup task tray
    let mut tray = TrayItem::new("nzskkserv", "tray-icon").unwrap();

    let (mes_sender, mes_reciver) = unbounded();
    let (start_send, start_recv) = unbounded::<()>();

    let s1 = mes_sender.clone();
    tray.add_menu_item("Show/Hide", move || {
        s1.send(Message::ShowHide).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Quit", move || {
        mes_sender.send(Message::Quit).unwrap();
    })
    .unwrap();

    // Load config and dicts
    let config = load_config().await?;
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
    {
        let server = Arc::clone(&server);
        tokio::task::spawn(async move {
            let res = server.start().await;
            if let Err(error) = res {
                error!("{}", error);
            }
        });
    }

    // Setup window
    let is_hidden = Arc::new(Mutex::new(true));
    let is_exit = Arc::new(Mutex::new(false));

    let options = eframe::NativeOptions::default();
    {
        let is_hidden = Arc::clone(&is_hidden);
        let is_exit = Arc::clone(&is_exit);
        thread::spawn(move || {
            let mut is_running = false;
            loop {
                match mes_reciver.recv() {
                    Ok(Message::Quit) => {
                        *is_exit.lock().unwrap() = true;
                        break;
                    }
                    Ok(Message::ShowHide) => {
                        let mut is_hidden = is_hidden.lock().unwrap();
                        if !is_running {
                            if *is_hidden {
                                start_send.send(()).unwrap();
                                is_running = true;
                            } else {
                                unreachable!()
                            }
                        }
                        *is_hidden = !*is_hidden;
                    }
                    _ => {}
                }
            }
        });
    }

    if let Ok(()) = start_recv.recv() {
        eframe::run_native(
            "nzskkserv-gui",
            options,
            Box::new(move |cc| {
                Box::new(app::App::new(
                    cc,
                    Arc::clone(&is_hidden),
                    Arc::clone(&is_exit),
                    Arc::clone(&server),
                ))
            }),
        );
        println!("exit")
    }

    Ok(())
}
