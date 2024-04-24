extern crate minigreplib;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigreplib::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("** Searching for {}, in file {} **\n", config.query, config.filename);

    if let Err(e) = minigreplib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
