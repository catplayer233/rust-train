mod type_alias_explorer;
mod never_return_type_explorer;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use crate::type_alias_explorer::{Score, Thunk};

    #[test]
    fn type_alias_test() {
        let x: i32 = 5;
        let y: Score = 7;
        println!("x = {}, y = {}, x+y = {}", x, y, x + y);

        let thunk: Thunk = Box::new(|| println!("hi"));
    }
}