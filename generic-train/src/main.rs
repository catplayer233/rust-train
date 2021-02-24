use std::process::exit;

use generic_explorer::generic_build;

use crate::lifetime_explorer::{reference_explore, StrHolder};
use crate::trait_explorer::{Summary, Tweet};

mod generic_explorer;
mod trait_explorer;
mod lifetime_explorer;

fn main() {
    let generic_struct = generic_build(123, "hello");
    let another_generic_struct = generic_build('A', 1.3);
    let mix_generic_struct = generic_struct.generic_method(another_generic_struct);

    println!("the mix_generic_struct is {:?}", mix_generic_struct);

    let tweet = Tweet {
        username: String::from("catplayer"),
        content: String::from("I just start to learn Rust and I think it's amazing for a Java coder."),
        reply: String::new(),
        retweet: String::new(),
    };

    println!("{}", tweet.summarize());
    let explore_str = reference_explore("hello", "world");
    println!("explore reference {}", explore_str);

    //struct lifetime use
    let test_string = String::from("test lifetime for struct. see the result");
    let target_reference_str = test_string.split(".").next().expect("can not get the str");
    let holder = StrHolder {
        content: target_reference_str
    };
    println!("get the result str: {}", holder.content);

    let target = all_in_one("hello", "rust", tweet);
    println!("get the final target: {}", target);
}

//in this function, we bound lifetime for the 2 &str input parameters, and we also use the generic bound for Summary
fn all_in_one<'a, T>(x: &'a str, y: &'a str, summary: T) -> &'a str where T: Summary {
    println!("do summarize: {}", summary.summarize());
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
