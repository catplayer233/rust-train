pub fn unsafe_raw_pointer() {
    //target object
    //&num and &mut num are all references
    let mut num = 6;

    let mut another_num = 10;
    //use the as keyword to cast a reference to into their corresponding raw pointer type
    let mut mutable_raw_pointer = &mut num as *mut i32;
    let mut another_mutable_raw_pointer = &mut another_num as *mut i32;

    //before
    unsafe {
        //if you want to access the pointer pointed value you should use unsafe code block
        println!(
            "before exchange: \n
        num:{},\n\
        another_num:{},\n\
        mutable_raw_pointer point to {},\n\
        another_mutable_raw_pointer point to {}\n",
            num, another_num, *mutable_raw_pointer, *another_mutable_raw_pointer
        );
    }

    num += 1;
    another_num -= 1;

    let temp_raw_pointer = mutable_raw_pointer;
    mutable_raw_pointer = another_mutable_raw_pointer;
    another_mutable_raw_pointer = temp_raw_pointer;

    unsafe {
        println!(
            "after exchange & value change: \n
        num:{},\n\
        another_num:{},\n\
        mutable_raw_pointer point to {},\n\
        another_mutable_raw_pointer point to {}",
            num, another_num, *mutable_raw_pointer, *another_mutable_raw_pointer
        );
    }
}
