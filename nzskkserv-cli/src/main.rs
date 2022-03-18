use std::net::{IpAddr, Ipv4Addr};

use clap::{Arg, Command};

use nzskkserv_server::server::Server;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let matches = Command::new("nzskkserv cli")
        .version("0.0.1")
        .author("nazo6")
        .arg(Arg::new("command").required(false))
        .get_matches();

    let server = Server::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2000);
    let result = server.start().await;
    match result {
        Ok(()) => (),
        Err(err) => println!("{}", err),
    }
}
