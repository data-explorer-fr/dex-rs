#![warn(clippy::all)]

use std::process;
use std::env;

// use clap::clap_app;
use dex_sources::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = dex_sources::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
