use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::string::String;

pub struct StartOptions {
    pub addr: String,
}

pub fn start(options: StartOptions) -> std::io::Result<()> {
    let listener = TcpListener::bind(&options.addr)?;
    println!("Starting server with address: {}", &options.addr);

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            let result = handle_connection(stream);
            result.unwrap()
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            println!("Invalid receive. Disconnecting...");
            return Ok(());
        }
        let string = String::from_utf8_lossy(&buffer[..]);

        match string.chars().next().borrow().unwrap() {
            '0' => {
                println!("0 Disconnect");
                return Ok(());
            }
            '1' => {
                println!("1 Convert");
            }
            '2' => {
                println!("2 Version");
                stream.write_all("nzskkserv/0.0.1".as_bytes())?;
            }
            '3' => {
                println!("3 host");
                stream.write_all(stream.local_addr()?.to_string().as_bytes())?;
            }
            '4' => {
                println!("4");
            }
            _ => {}
        }

        stream.flush()?;
    }
}
