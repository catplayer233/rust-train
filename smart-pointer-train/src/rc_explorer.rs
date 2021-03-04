use std::rc::Rc;

pub enum RCList<T> {
    Cons(T, Rc<RCList<T>>),
    Nil,
}