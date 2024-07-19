use std::env;
use std::process;

use miniproject::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // run(config);
    if let Err(e) = miniproject::run(config) {
        eprintln!("Problem with the code {e}");
        process::exit(1);
    }
}


