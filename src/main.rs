use std::{ env, process };

use crablibs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = crablibs::run(config) {
        eprintln!("Error: {}", e);

        process::exit(1);
    }
}
