use std::ops::Add;

#[derive(Debug,PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

//the Add source code:
//the Rhs=Self is called default type parameters
//this can specify the target generic type's default type
//if we not specify the type, Self will be the default type
/* trait Add<Rhs=Self> {
     type Output;

     fn add(self, rhs: Rhs) -> Self::Output;
 }*/

//you also can specify the type

struct Meters(u32);

struct Millimeters(u32);

//there you can specify the type will be used to add with a Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}
