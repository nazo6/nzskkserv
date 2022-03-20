use std::net::{IpAddr, Ipv4Addr};

use clap::{Arg, Command};


#[tokio::main]
async fn main() {
    let server = Server::new();
    dbg!("{:?}", server)
}
