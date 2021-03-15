mod associated_type_explorer;
mod operator_overloading_explorer;
mod fully_qualified_syntax_explorer;
mod super_traits_explorer;
mod newtype_pattern_explorer;

fn main() {}


#[cfg(test)]
mod test {
    use crate::associated_type_explorer::{Animal, AnimalKind, Human, MarkerResolution};
    use crate::fully_qualified_syntax_explorer::{Barker, Dog, Person, Pilot, Wizard};
    use crate::newtype_pattern_explorer::Wrapper;
    use crate::operator_overloading_explorer::Point;
    use crate::super_traits_explorer::{Message, OutlinePrint};

    #[test]
    fn associated_type_test() {
        let catplayer = Human::build(String::from("catplayer"), 18);
        let eagle = Animal::build(AnimalKind::FLY, String::from("eagle"));
        println!("the human is: {:?}, the marker: {}", catplayer, catplayer.get_marker());
        println!("the animal is:{:?}, the marker: {}", eagle, eagle.get_marker());
    }

    #[test]
    fn operator_overloading() {
        let a = Point {
            x: 1,
            y: 1,
        };
        let b = Point {
            x: 3,
            y: 7,
        };
        assert_eq!(a + b, Point {
            x: 4,
            y: 8,
        });
    }

    #[test]
    fn same_method_name_test() {
        //fully qualified syntax:
        //<Type as Trait>::function(receiver_if_method, next_arg, ...);
        let person = Person;
        //associated function use the syntax:
        //(Struct/trait name)::associated function name(&(target value),...(other parameters))
        //this syntax can declare the specific associated functions when you invoked
        //this is very helpful when your value has multiple same functions which are belonged to different traits/struct
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();

        //when the associate function doesn't has the self parameter
        //we should use syntax to declare the specific trait's function:
        //<struct name as trait name>::associated function name(...(other parameter))
        //the struct dog's bark
        println!("{}", Dog::bark());
        //invoke the Barker's associated function
        println!("{}", <Dog as Barker>::bark());
    }

    #[test]
    fn super_traits_test() {
        let message = Message::build(String::from("test"), String::from("test the super traits features"));
        message.outline_print();
    }

    #[test]
    fn newtype_pattern_test() {
        let wrapper = Wrapper(vec![String::from("hello"), String::from("rust")]);
        println!("wrapper = {}", wrapper);
    }
}
