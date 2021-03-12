static mut COUNTER: u32 = 0;

pub fn modify_static_variable(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
