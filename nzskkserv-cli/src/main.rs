use nzskkserv_lib;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("nzskkserv cli")
        .version("0.0.1")
        .author("nazo6")
        .arg(Arg::with_name("command").required(false))
        .get_matches();

    let opts = nzskkserv_lib::StartOptions {
        addr: "localhost:1000".to_string(),
    };
    nzskkserv_lib::start(opts).unwrap_err();
}
