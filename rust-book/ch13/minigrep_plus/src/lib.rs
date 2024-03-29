// use lib to read file
use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't git a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't git a query string"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query = query.to_lowercase();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    contents
        .lines()
        .filter(|line| line.contains(&query.to_lowercase()))
        .collect()

    // results
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "KUN";
        let contents = "\
KUN IS CUTEST!
Kun IS MOEMOE!
kun IS PERSEVERE!";
        assert_eq!(vec!["KUN IS CUTEST!"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "cutest";
        let contents = "\
KUN IS CUTEST!
Kun IS MOEMOE!
kun IS PERSEVERE!";

        assert_eq!(
            vec!["KUN IS CUTEST!"],
            search_case_insensitive(query, contents)
        );
    }
}
