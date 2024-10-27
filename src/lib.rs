use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() > 3 {
            return Err("Not enought arguments");
        }
        args.next(); // skip first one
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("No file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: query.to_string(),
            file_name: file_name.to_string(),
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contens: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contens.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contens
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contens: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contens.lines() {
        if line.to_lowercase().contains(query.as_str()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
Safe, fast, productive.
            Pick three.
DuCt";
        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
            Rust:
Safe, fast, productive.
            Pick three.
DuCt";
        assert_eq!(
            vec!["Safe, fast, productive.", "DuCt"],
            search_case_insensitive(query, contents)
        );
    }
}
