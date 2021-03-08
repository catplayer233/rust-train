fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::test::Whatever::{OTHERS, STRUCT, TUPLE, UNIT};

    enum Whatever {
        UNIT,
        TUPLE(i32, i32),
        STRUCT {
            a: i32,
            b: i32,
        },
        OTHERS,
    }

    #[test]
    fn normal() {
        let whatever = STRUCT {
            a: 1,
            b: 2,
        };
        match whatever {
            UNIT => println!("get the type unit"),
            TUPLE(a, b) => println!("get the type tuple with value:a = {}, b = {}", a, b),
            STRUCT { a: x, b: y } => println!("get the type struct with value: x={}, y={}", x, y),
            //_ declare anything of the whatever enum, and the _ not a specific value of the whatever
            _ => println!("others"),
        }
    }

    #[test]
    fn if_let_match() {
        let whatever = TUPLE(1, 3);

        if let OTHERS = whatever {
            println!("others");
        } else if let TUPLE(a, b) = whatever {
            println!("get the type tuple with value:a = {}, b = {}", a, b);
        } else {
            println!("nothing happened")
        }
    }

    #[test]
    fn while_match() {
        let mut whatever_vec = vec![TUPLE(1, 3), STRUCT { a: 3, b: 8 }, UNIT, OTHERS];

        while let Some(whatever) = whatever_vec.pop() {
            match whatever {
                UNIT => println!("get the type unit"),
                TUPLE(a, b) => println!("get the type tuple with value:a = {}, b = {}", a, b),
                STRUCT { a, b } => println!("get the type struct with value: x={}, y={}", a, b),
                //_ declare anything of the whatever enum, and the _ not a specific value of the whatever
                _ => println!("others"),
            }
        }
    }

    #[test]
    fn for_match() {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    #[test]
    fn multi_match() {
        let x = 1;
        match x {
            //use the | operator to declare multi value to match the target
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn range_match() {
        let x = 5;
        match x {
            //use the ..= to declare range(inclusive)
            //only valid for numeric values and char
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }


    enum RangeEnum {
        None,
        Range(i32),
    }

    #[test]
    fn bind_operator() {
        let range = RangeEnum::Range(6);

        match range {
            //use the @ operator, you can bind the pattern's variable to a specific range condition
            RangeEnum::Range(a @ 1..=9) => println!("a is between 1 and 9, and value is {}", a),
            RangeEnum::Range(_) => println!("some other value"),
            RangeEnum::None => println!("nothing"),
        }
    }
}