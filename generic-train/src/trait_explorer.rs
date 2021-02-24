//!define a trait, just like write a java interface.
//! you should specify some functions for this trait,
//! every implementation should implement these functions.
//! a standard function should specify function signature without code block
use std::fmt::{Debug, Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsLetter {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub other: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}


//when your struct need to impl the trait
//you should write like this:
//```
//impl TRAIT_NAME for STRUCT_NAME{
//...//functions
// }
// ```
impl Summary for NewsLetter {
    fn summarize(&self) -> String {
        format!("{}, by{} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//you can do this when the trait or struct inside current crate
impl<T> Summary for Vec<T> {
    fn summarize(&self) -> String {
        String::new()
    }
}

//you can't do this, because Display and Vec are all outside current crate
// impl<T> Display for Vec<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<T, String> {
//         Err(String::new())
//     }
// }

//pass trait as parameter
//you can use the keyword impl to pass a trait
pub fn notify(summary: &impl Summary) {
    println!("{}", summary.summarize())
}

//you also can use the trait bound syntax to do the same thing
//this is a better way when your parameter has more than one with the same trait
pub fn notify_bound<T: Summary>(summary: &T) {
    println!("{}", summary.summarize());
}


//you can bound a parameter with multiple traits
//use the + operator
pub fn notify_multiple_bound<T: Summary + Display>(trait_impl: &T) {}

//where syntax can also do trait bound
//this syntax make your function signature more clear
pub fn where_bound<T, U>(bound_argument_t: T, bound_parameter_u: U) where T: Summary + Debug, U: Debug + Clone {}
