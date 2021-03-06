extern crate minigrepyay;

use std::env;
use std::process;

use minigrepyay::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrepyay::run(config) {
        eprintln!("Application error: {}", err);

        process::exit(1);
    }
}