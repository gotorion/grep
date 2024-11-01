use std::env;
use std::process;

use grep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = grep::run(config) {
        // we do not care value of Result(Ok), hence if let is better choice
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
