mod trait_object_explorer;
mod state_pattern;

fn main() {}


#[cfg(test)]
mod test {
    use crate::trait_object_explorer::{Draw, Screen, ScreenGeneric};

    struct Keyboard;

    impl Draw for Keyboard {
        fn draw(&self) {
            println!("use keyboard to draw");
        }
    }

    struct Mouse;

    impl Draw for Mouse {
        fn draw(&self) {
            println!("use mouse to draw");
        }
    }

    #[test]
    fn generic_test() {
        let keyboard = Keyboard {};
        let mouse = Mouse {};
        let screen_generic = ScreenGeneric {
            //you can save different trait object types for generic struct
            // components: vec![keyboard, mouse]
            components: vec![keyboard]
        };
        screen_generic.run();
    }

    #[test]
    fn trait_object_test() {
        let keyboard = Keyboard {};
        let mouse = Mouse {};
        let screen = Screen {
            //you can save different trait object types for generic struct
            // components: vec![keyboard, mouse]
            components: vec![Box::new(keyboard), Box::new(mouse)]
        };
        screen.run();
    }

    #[test]
    fn state_pattern_test() {


    }
}

