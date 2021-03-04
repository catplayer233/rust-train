use std::cell::RefCell;
use std::rc::Rc;

use crate::reference_cycle_explorer::List::Cons;
use crate::reference_cycle_explorer::List::Nil;

#[derive(Debug)]
pub enum List {
    Cons(RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(item) => Some(item),
            Nil => None,
        }
    }
}
