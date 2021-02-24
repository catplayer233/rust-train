use std::error::Error;
use std::fs;

pub struct SearchConfiguration {
    search_word: String,
    search_file: String,
}

impl SearchConfiguration {
    pub fn new(arguments: &[String]) -> Result<SearchConfiguration, &str> {
        if arguments.len() < 3 {
            return Err("not enough arguments");
        }

        let word = arguments[1].clone();
        let file_name = arguments[2].clone();
        Ok(SearchConfiguration {
            search_word: word,
            search_file: file_name,
        })
    }
}

pub fn run(search_config: SearchConfiguration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(search_config.search_file)?;

    for searched_line in search(&search_config.search_word, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_result() {
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}