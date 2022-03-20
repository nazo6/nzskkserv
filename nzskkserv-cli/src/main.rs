use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use clap::{Arg, Command};
use log::info;
use nzskkserv_core::{Dict, config::{DictPath, Encoding}};

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let server = &nzskkserv_core::manager::Manager::new(LOCALHOST, 2000);
    server.load_config(None).await;
    server.set_dicts(vec![Dict::DictPath(DictPath {
        path: "/home/nazo/.config/nzskkserv/SKK-JISYO.ML".to_string(),
        encoding: Some(Encoding::Eucjp)
    })]).await;
    server.set_google_cgi(true).await;
    let start_server = || async {
        server.start().await;
        info!("Server exited")
    };

    tokio::join! {
        start_server(),
    };
}
