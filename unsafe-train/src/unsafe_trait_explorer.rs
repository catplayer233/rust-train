//declare a trait is a unsafe trait,
//add keyword unsafe when you define the trait
pub unsafe trait UnsafeTrait {
    unsafe fn do_dangerously(&self);
}

//the implementation should add unsafe when the trait is a unsafe trait
unsafe impl UnsafeTrait for String {
    unsafe fn do_dangerously(&self) {
        println!("get the string:{}", self);
    }
}
