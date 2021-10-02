use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::string::String;

pub struct StartOptions {
    pub addr: String,
}

pub fn start(options: StartOptions) -> std::io::Result<()> {
    let listener = TcpListener::bind(&options.addr)?;
    println!("{}", &options.addr);

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            handle_connection(stream);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    println!("!!!");
    println!("{}", stream.local_addr().unwrap());
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
