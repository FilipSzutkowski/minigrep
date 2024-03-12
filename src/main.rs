use minigrep::{run, Config};
use std::{env::args, process};

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with parsing: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
