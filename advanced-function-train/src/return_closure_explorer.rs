//you can't return a closure directly instead you should use the trait object for this purpose
pub fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}