use std::env;
use std::process;
use std::result::Result::Err;

use minigrep::{run, SearchConfiguration};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let search_config = SearchConfiguration::new(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(search_config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}
