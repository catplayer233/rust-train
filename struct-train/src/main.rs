#[derive(Debug)]
struct Retangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct TupleStruct(u32, u32);

impl TupleStruct {
    fn tuple_area(&self) -> u32 {
        self.0 * self.1
    }
}

impl Retangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold_target_retangle(&self, target_retangle: &Retangle) -> bool {
        self.width >= target_retangle.width && self.height >= target_retangle.height
    }

    fn instance(width: u32, height: u32) -> Retangle {
        Retangle { width, height }
    }

    fn instance_by_other(width: u32, original_retangle: Retangle) -> Retangle {
        Retangle {
            width,
            ..original_retangle
        }
    }
}

fn main() {
    let retangle = Retangle {
        width: 32,
        height: 80,
    };
    println!(
        "the retangle is {:#?}, and area is {}",
        retangle,
        retangle.area()
    );

    let tuple = TupleStruct(30, 80);

    println!(
        "the retangle is {:#?}, and area is {}",
        tuple,
        tuple.tuple_area()
    );

    let another_retangle = build_retangle(20, 30);

    println!(
        "retangle is {:#?}, target retangle is {:#?} and hold result is {}",
        retangle,
        another_retangle,
        retangle.can_hold_target_retangle(&another_retangle)
    );

    let retangle_instance = Retangle::instance(30, 60);
    println!(
        "use the associated function, get the retangle instance:{:#?}",
        retangle_instance
    );

    let retangle_instance = Retangle::instance_by_other(29, retangle_instance);

    println!("reinstance the retangle instance:{:#?}", retangle_instance);
}

fn build_retangle(width: u32, height: u32) -> Retangle {
    Retangle { width, height }
}
