use minigrep;

#[test]
fn case_sensitive() {
    let query = "duct";
    // backslash after opening " tells Rust not to put a newline before the first line of the string.
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
}

#[test]
fn case_insensitive()
{
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(vec!["Rust:", "Trust me."], minigrep::search_case_insensitive(query, contents));
}