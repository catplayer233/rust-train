//!
//! 5 ways to use unsafe in rust
//! - Dereference a raw pointer
//! - Call an unsafe function or method
//! - Access or modify a mutable static variable
//! - Implement an unsafe trait
//! - Access fields of unions
//!
mod access_modify_static_mutable_variable_explorer;
mod dereference_raw_pointer_explorer;
mod unsafe_function_explorer;
mod unsafe_trait_explorer;

fn main() {}

#[cfg(test)]
mod test {
    use crate::access_modify_static_mutable_variable_explorer::modify_static_variable;
    use crate::dereference_raw_pointer_explorer::unsafe_raw_pointer;
    use crate::unsafe_function_explorer::{call_extern_func, dangerous};
    use crate::unsafe_trait_explorer::UnsafeTrait;

    #[test]
    fn raw_pointer() {
        unsafe_raw_pointer();
    }

    #[test]
    fn invoke_unsafe_function() {
        unsafe { dangerous() }
    }

    #[test]
    fn invoke_extern_function() {
        call_extern_func();
    }

    #[test]
    fn invoke_modify_static_variable() {
        modify_static_variable(1);
    }

    #[test]
    fn invoke_unsafe_trait_func() {
        let unsafe_string = String::from("hello");
        unsafe {
            unsafe_string.do_dangerously();
        }
    }
}
