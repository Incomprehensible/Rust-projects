use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.file);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Could not read contents of the file: {}", e);
        process::exit(1);
    }
}
