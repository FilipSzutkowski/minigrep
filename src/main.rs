use std::{env::args, fs::read_to_string};

fn main() {
    let args: Vec<String> = args().collect();
    let Config {
        query: _query,
        file_path,
    } = Config::new(&args);

    let contents = read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args.get(1).expect("Specify the search query.").clone();
        let file_path = args.get(2).expect("Specify the file path.").clone();

        println!("Searching for '{query}'");
        println!("In file '{file_path}'");

        Config { query, file_path }
    }
}
