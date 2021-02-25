use std::env;
use std::error::Error;
use std::fs;

pub struct SearchConfiguration {
    search_word: String,
    search_file: String,
    case_sensitive: bool,
}

impl SearchConfiguration {
    pub fn new(arguments: &[String]) -> Result<SearchConfiguration, &str> {
        if arguments.len() < 3 {
            return Err("not enough arguments");
        }

        let word = arguments[1].clone();
        let file_name = arguments[2].clone();

        //true->case sensitive, false->case insensitive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(SearchConfiguration {
            search_word: word,
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
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let lowercase_query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            result.push(line);
        }
    }
    result
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