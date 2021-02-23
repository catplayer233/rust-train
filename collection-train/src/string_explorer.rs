pub fn explore() {
    let s1 = String::from("你好");
    let s2 = String::from(" in Rust world");
    //when you use + operator, this means you invoke the add function in string.rs
    // the function require:
    // self: string and take the ownership
    // another: &str
    //so there s1 must move the ownership, s2 use & operator for declaring as a &str
    //and the value will be returned as a String type
    let mut s1 = s1 + &s2;
    //as same as above line,
    // because the rust will dynamic change &string to &string[..] and this is &str
    // let mut s1 = s1 + &s2[..];
    s1.push_str("\n");
    println!("s1 is {}, s2 is {}", s1, s2);
    //you can't do this
    //because in rust string store word via UTF-8, so the specific index value may be is not a valid
    //word. So you can't use index method to get the value with specific index when you use String.
    // let index_value = s1[0];
    //get the actual word in String, you can call chars method
    for char in s1.chars() {
        println!("{}", char);
    }
    //and you also can use bytes method to get the bytes in String
    for byte in s1.bytes() {
        println!("{}", byte);
    };
}