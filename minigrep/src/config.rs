use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    //pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    pub fn parse_args(mut args: env::Args) -> Result<Config, &'static str> {
        // skip executable name
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing required arguments: [search_expr] [filename]")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing required arguments: search_expr [filename]")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}
