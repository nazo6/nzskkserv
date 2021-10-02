use clap::{App, Arg};

fn main() {
    let matches = App::new("nzskkserv cli")
        .version("0.0.1")
        .author("nazo6")
        .arg(Arg::with_name("command").required(false))
        .get_matches();

    let opts = nzskkserv::StartOptions {
        addr: "127.0.0.1:1000".to_string(),
    };
    nzskkserv::start(opts).unwrap_err();
}
