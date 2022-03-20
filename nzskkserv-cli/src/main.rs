use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use clap::{Arg, Command};
use log::info;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let server = &nzskkserv_core::Manager::new(LOCALHOST, 2000);
    let start_server = || async {
        server.start().await;
        info!("Server exited")
    };


    tokio::join! {
        start_server(),
    };
}
