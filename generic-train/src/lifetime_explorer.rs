//the 'a declare the x, y and the return value has the same lifetime
pub fn reference_explore<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//when we construct a struct and the filed declare a reference,
// we should use lifetime specifier to declare the filed has the same lifetime as the outside reference
pub struct StrHolder<'a> {
    pub content: &'a str
}