use std::fmt::{Display, Formatter};
use std::fmt;

//declare the OutlinePrint has depend on super trait: Display
//so the parameter self can has the to_string function
//if your trait has multiple super traits you should use + to add them
//eg. OutlinePrint: Display+Add
pub trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

pub struct Message {
    name: String,
    content: String,
}

impl Message {
    pub fn build(name: String, content: String) -> Self {
        Message {
            name,
            content,
        }
    }
}

//you can not implement the OutlinePrint because the Message not implement the Display trait
// impl OutlinePrint for Message {}


impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.name, self.content)
    }
}

//now you can implement the OutlinePrint because the Message has implemented the Display trait
impl OutlinePrint for Message {}
