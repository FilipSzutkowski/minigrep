use std::{env::args, fs::read_to_string, process};

fn main() {
    let args: Vec<String> = args().collect();
    let Config {
        query: _query,
        file_path,
    } = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with parsing: {err}");
        process::exit(1)
    });

    let contents = read_to_string(file_path).unwrap_or_else(|err| {
        println!("Problem with reading file: {err}");
        process::exit(1)
    });

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        println!("Searching for '{query}'");
        println!("In file '{file_path}'");

        Ok(Config { query, file_path })
    }
}
