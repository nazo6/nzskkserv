use std::net::{IpAddr, Ipv4Addr};

use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let mut server = nzskkserv_core::Server::default();
    server.start(IpAddr::V4(Ipv4Addr::LOCALHOST), 2000).await;
}
