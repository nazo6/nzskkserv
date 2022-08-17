#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::net::{IpAddr, Ipv4Addr};
const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tauri::command]
async fn start_server() {
    println!("Starting server...");
    let server = nzskkserv_core::Server::new(
        LOCALHOST,
        1178,
        Vec::new(),
        true,
        nzskkserv_core::Encoding::Utf8,
    );
    server.start().await;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
