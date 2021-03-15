//define type alias
//type ALIAS_NAME = exists type name
pub type Score = i32;

//you can bound a complicated type to a alias to reduce code complexity
pub type Thunk = Box<dyn Fn() + Send + 'static>;