use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!(
        "\x1b[32mLooking for '\x1b[33m{}\x1b[32m' in '\x1b[33m{}\x1b[0m'",
        config.query, config.file_path
    );

    let contents: String = fs::read_to_string(&config.file_path)?;
    println!("Looking within the text:\n\x1b[0m{}", contents);

    println!("\n\n\x1b[32mFound lines:\x1b[0m");

    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn parse_arguments(arguments: &[String]) -> Result<Config, &str> {
        if arguments.len() < 3 {
            let message = "not enough arguments (2 required)";
            return Err(message);
        } else if arguments.len() > 3 {
            let message = "too many arguments (2 required)";
            return Err(message);
        }

        let query = arguments[1].clone();
        let file_path = arguments[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    return result;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_ascii_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_ascii_lowercase().contains(&query) {
            result.push(line);
        }
    }

    return result;
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
        );
    }
}
