use std::net::{IpAddr, Ipv4Addr};

use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("nzskkserv cli")
        .version("0.0.1")
        .author("nazo6")
        .arg(Arg::with_name("command").required(false))
        .get_matches();

    let server = nzskkserv_server::Server::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2000);
    let result = server.start().await;
    match result {
        Ok(()) => (),
        Err(err) => println!("{}", err),
    }
}
