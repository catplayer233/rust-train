use std::env;
use std::process;
use std::result::Result::Err;

use minigrep::{run, SearchConfiguration};

fn main() {
    let search_config = SearchConfiguration::new(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(search_config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
