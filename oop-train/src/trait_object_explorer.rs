pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    //keyword dyn declare the target is a Draw trait object
    //and we should use this with pointer: & or smart pointer
    //you can provide different trait objects which don't
    //need to be the same type
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct ScreenGeneric<T> where T: Draw {
    pub components: Vec<T>,

}

impl<T> ScreenGeneric<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

