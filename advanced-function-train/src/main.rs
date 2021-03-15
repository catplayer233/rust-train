mod function_pointer_explorer;
mod return_closure_explorer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::function_pointer_explorer::{add_one, build_set_status, do_twice, two_kind_function_usage};

    #[test]
    fn function_pointer_test() {
        let answer = do_twice(add_one, 5);
        println!("the answer is {}", answer);

        two_kind_function_usage();
        build_set_status();
    }
}