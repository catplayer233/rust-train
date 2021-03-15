//!# NewType Pattern
//!if we want to implement a trait which was not belonged to local crate
//!for a struct which also was not belonged to local crate(normally, this is not allowed.)
//!
//!we can use the newtype pattern
//! use a tuple struct wrap the target struct(not belonged to local crate) and implement the trait
//! ## eg. Vec implement Display
use std::fmt;
use std::fmt::Formatter;

pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}