pub unsafe fn dangerous() {
    println!("you call the unsafe function");
}

//when you want to call other language's functions, you should use
//the keyword extern(Foreign Function Interface)
//the C means we call a c lang's function among ABI(application binary interface)
//at assembly level
//function which was defined in extern also is unsafe function
//if you want to use this function, you should surround it with unsafe code block
extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn call_extern_func() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
