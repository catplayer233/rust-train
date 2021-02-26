use std::env;
use std::env::Args;
use std::error::Error;
use std::fs;
use std::fs::read;
use std::iter::Skip;

pub struct SearchConfiguration {
    search_word: String,
    search_file: String,
    case_sensitive: bool,
}

impl SearchConfiguration {
    pub fn new<T>(mut arguments_iter: T) -> Result<SearchConfiguration, &'static str> where T: Iterator<Item=String> {
        let mut get_arg_fn = |error_message| {
            match arguments_iter.next() {
                Some(arg) => Ok(arg),
                None => Err(error_message),
            }
        };
        let query = get_arg_fn("Didn't get a query string")?;
        let file_name = get_arg_fn("Didn't get a file name")?;

        //true->case sensitive, false->case insensitive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(SearchConfiguration {
            search_word: query,
            search_file: file_name,
            case_sensitive,
        })
    }
}

pub fn run(search_config: SearchConfiguration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(search_config.search_file)?;
    let query = &search_config.search_word;
    let case_sensitive = search_config.case_sensitive;

    let result = if case_sensitive {
        search(query, &contents)
    } else {
        search_case_insensitive(query, &contents)
    };

    for searched_line in result {
        println!("{}", searched_line);
    };

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_result() {
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "RusT";
        let content = "\nRust:\nsafe, fast, productive.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}