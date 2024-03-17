use std::{env, error::Error, fs::read_to_string, sync::Arc, thread};

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
        let contents = read_to_string(&config.file_paths[0])?;
        run_search(&config, &contents);

        return Ok(());
    }

    let config = Arc::new(config);
    let mut handles = vec![];

    for file_idx in 0..files_count {
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let path = &config.file_paths[file_idx];
            let file_read = read_to_string(path);

            println!("[{path}]: ");
            match file_read {
                Ok(contents) => run_search(&config, &contents),
                Err(e) => eprintln!("Error when reading file: {e}"),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    Ok(())
}

pub fn run_search(config: &Config, contents: &str) {
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}")
    }
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
