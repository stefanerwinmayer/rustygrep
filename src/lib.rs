use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file name provided"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "truth";
        let contents = "\
Truth is:
When you have eliminated all which is impossible,
then whatever remains, however improbable,
must be the truth.";

        assert_eq!(vec!["must be the truth."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "trUth";
        let contents = "\
Truth is:
When you have eliminated all which is impossible,
then whatever remains, however improbable,
must be the truth.";

        assert_eq!(
            vec!["Truth is:", "must be the truth."],
            search_case_insensitive(query, contents)
        );
    }
}
