pub fn add_one(x: i32) -> i32 {
    x + 1
}

//fn(arg-type...)->return-type as a function pointer
//fn implemented Fn, FnMut, and FnOnce so the trait declare can also use the function as well
pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn two_kind_function_usage() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    //FnMut trait also be implemented by fn pointer, so you can use a function
    let another_way: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

enum Status {
    Value(u32),
    Stop,
}

pub fn build_set_status() {
    //Value(u32) also is a function which will new a tuple of Value
    //so you can use Status::Value as the fn pointer
    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
}



