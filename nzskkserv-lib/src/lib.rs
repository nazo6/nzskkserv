use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    let mut stream = std::io::BufReader::new(stream);

    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    println!(stream.get())
}

pub fn start() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
