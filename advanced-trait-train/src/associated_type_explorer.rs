//! # this is a trait which has an associated type
//! the difference between associated type and generic is
//!
//! the associate type make sure that the target trait will only has one implementation for a specific struct
//!
//! and the generic may be has multiple implementations for one struct
//!
//! so you should choose the right one for your programs, just need one type or can have different behaviors

pub trait MarkerResolution {
    type ResolutionType;

    fn get_marker(&self) -> Self::ResolutionType;
}

#[derive(Debug)]
pub struct Human {
    name: String,
    age: u8,
}

#[derive(Debug)]
pub enum AnimalKind {
    SWIM,
    FLY,
    WALK,
}

#[derive(Debug)]
pub struct Animal {
    kind: AnimalKind,
    name: String,
}

impl Human {
    pub fn build(name: String, age: u8) -> Self {
        Human {
            name,
            age,
        }
    }
}

impl Animal {
    pub fn build(kind: AnimalKind, name: String) -> Self {
        Animal {
            kind,
            name,
        }
    }
}

impl AnimalKind {
    fn kind_display(&self) -> String {
        match self {
            AnimalKind::SWIM => String::from("swim"),
            AnimalKind::FLY => String::from("fly"),
            AnimalKind::WALK => String::from("walk"),
        }
    }
}

impl MarkerResolution for Human {
    type ResolutionType = String;

    fn get_marker(&self) -> Self::ResolutionType {
        self.name.clone()
    }
}

impl MarkerResolution for Animal {
    type ResolutionType = String;

    fn get_marker(&self) -> Self::ResolutionType {
        self.kind.kind_display()
    }
}