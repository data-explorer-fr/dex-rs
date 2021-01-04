#![warn(clippy::all)]

use std::error::Error;
use std::fs;

pub mod config;

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    println!("query:\n{}", config.query);

    Ok(())
}
