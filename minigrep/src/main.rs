use std::env;
use std::process;

use minigrep::config::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
    let args = env::args(); // iter

    let config = Config::parse_args(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing command line: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
