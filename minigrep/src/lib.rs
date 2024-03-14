use std::fs;
use std::error::Error;

pub mod config;

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> 
{
    let contents = fs::read_to_string(config.filename)?;

    let matches = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in matches {
        println!("{}", line);
    }

    Ok(())
}

// lifetime parameter 'a
// tells Rust that this function returns a Vector of slices whose lifetime matches the lifetime of the "contents" string
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    /* 
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
    */

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    /* 
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }

    matches
    */

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}