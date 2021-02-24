//declare a generic struct
#[derive(Debug)]
pub struct GenericStruct<T, U> {
    generic_a: T,
    generic_b: U,
}

//declare generic impl code block
impl<T, U> GenericStruct<T, U> {
    //declare a generic method
    pub fn generic_method<V, W>(self, another_struct: GenericStruct<V, W>) -> GenericStruct<T, W> {
        GenericStruct {
            generic_a: self.generic_a,
            generic_b: another_struct.generic_b,
        }
    }
}

//declare method
pub fn generic_build<T, U>(generic_a: T, generic_b: U) -> GenericStruct<T, U> {
    GenericStruct { generic_a, generic_b }
}