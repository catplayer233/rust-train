pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Person;

//Pilot fly
impl Pilot for Person {
    fn fly(&self) {
        println!("This is your caption speaking");
    }
}

//Wizard fly
impl Wizard for Person {
    fn fly(&self) {
        println!("Up!");
    }
}

//Human fly
impl Person {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


pub trait Barker {
    fn bark() -> String;
}

pub struct Dog;

impl Dog {
    pub fn bark() -> String {
        String::from("bark by a dog")
    }
}

impl Barker for Dog {
    fn bark() -> String {
        String::from("bark by a barker")
    }
}