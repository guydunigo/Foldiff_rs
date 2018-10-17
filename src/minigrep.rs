extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        /*
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = String::from(args[1]);
        let filename = String::from(args[2]);
        */

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, regex::Error> {
    let regex = Regex::new(query)?;

    let res = contents.lines()
        .filter(|line| {
            regex.is_match(line)
        })
        .collect();

    Ok(res)
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, regex::Error> {
    let case_insensitive_query = format!("(?i){}", query);

    search(&case_insensitive_query, contents)
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // Open file

    println!("In file `{}`", config.filename);

    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // println!("With text :\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)?
    } else {
        search_case_insensitive(&config.query, &contents)?
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).unwrap_or_else(|err| {
                panic!("Error while searching : {}", err)
            })
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, contents).unwrap_or_else(|err| {
                panic!("Error while searching : {}", err)
            })
        );
    }
}
