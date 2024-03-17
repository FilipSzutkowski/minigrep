use std::{
    env,
    error::Error,
    fs::read_to_string,
    io::{stdout, Write},
    sync::Arc,
    thread,
};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Specify a query string"),
        };

        let min_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Specify at least one file path."),
        };

        let mut all_paths: Vec<String> = args.into_iter().collect();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        all_paths.push(min_path);

        Ok(Config {
            query,
            file_paths: all_paths,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let files_count = config.file_paths.len();

    if files_count == 1 {
        let path = &config.file_paths[0];
        let contents = read_to_string(path)?;
        run_search(&config, &contents);

        return Ok(());
    }

    // Config will have multiple borrowers
    let config = Arc::new(config);
    let mut handles = vec![];

    for file_idx in 0..files_count {
        // Getting a new referance to config for moving
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let path = &config.file_paths[file_idx];
            let file_read = read_to_string(path);
            let content: Box<String>;

            match file_read {
                // Box for moving ownership upwards
                Ok(read_contents) => content = Box::new(read_contents),
                Err(e) => panic!("Error when reading '{path}': {e}"),
            };

            let search_result = run_search(&config, &content);

            // Locking stdout, so that the printed result
            // appears in right order
            let mut std_lock = stdout().lock();

            writeln!(std_lock, "\n[{path}]: ").unwrap();

            for result in search_result {
                writeln!(std_lock, "{result}").unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

pub fn run_search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    result
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();

    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
          Rust:
          safe, fast, productive.
          Pick three.
          Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
          Rust:
          safe, fast, productive.
          Pick three.
          Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
