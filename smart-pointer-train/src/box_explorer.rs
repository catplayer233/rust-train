use std::boxed::Box;

pub fn new_box<T>(target: T) -> Box<T> {
    Box::new(target)
}

pub enum List<T> {
    //you can not use the List directly,
    //because rust compiler can not figure out the memory space the struct will take
    //so you should use Box, Box means a pointer and the space can be confirmed
    Cons(T, Box<List<T>>),
    Nil,
}