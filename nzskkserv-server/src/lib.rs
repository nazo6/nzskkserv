use anyhow::Result;
use std::net::IpAddr;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    address: IpAddr,
    port: u16,
}

impl Server {
    pub fn new(address: IpAddr, port: u16) -> Server {
        Server { address, port }
    }
    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind((self.address, self.port)).await?;
        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                println!("Connected");
                process(socket).await
            });
        }
    }
}

async fn process(mut stream: TcpStream) {
    let mut buf = vec![0; 1];
    match stream.read(&mut buf).await {
        Ok(0) => {
            println!("Connect end");
        }
        Ok(n) => {
            println!("{n}");
            let data = &buf[..n];
            let str = String::from_utf8(data.to_vec()).unwrap();
            println!("{str}")
        }
        Err(_) => {
            println!("Error");
        }
    }
}
