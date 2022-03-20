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

    let wait_3_secs = || async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        info!("3secs");
        server.stop().await;
    };

    tokio::join! {
        start_server(),
        wait_3_secs()
    };
}
