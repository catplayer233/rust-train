use MessedType::{A, B, C};

pub fn explore() {
    //macro init
    let macro_vec = vec![10, 20, 40];

    println!("get the macro_vec value: {}", macro_vec[0]);

    //normal api init
    //there we specify the type, because the new fn not specify the actual type the vector can hold
    //so we specify manually and the compiler can infer the vector's type
    //use mut keyword declare the new_vec is mutable
    let mut new_vec: Vec<i32> = Vec::new();

    new_vec.push(1);
    new_vec.push(3);

    let value_option = new_vec.get(0);
    if let Some(value) = value_option {
        println!("get the new_vec value: {}", value);
    }

    //use index
    let index_vec_value = &new_vec[0];
    //this cause panic!
    // let index_not_exists_vec_value = &new_vec[100];

    //the index_vec_value has the immutable borrow, an later we used this value, so you can not call push to modify
    // new_vec.push(12);

    println!("get the index_vec_value: {}", index_vec_value);

    //this return an option type of None
    let _get_not_exists_vec_value = new_vec.get(100);

    //iterating
    //we should specify reference not the new_vec self, for keeping the ownership
    for value in &new_vec {
        println!("{}", value);
    }
    println!("new_vec: {:?}", new_vec);

    //use * to dereference
    for mutable_value in &mut new_vec {
        *mutable_value += 233;
    }
    println!("new_vec after iter modify: {:?}", new_vec);

    //enum
    let enum_vec = vec![A, B(32), C { name: String::from("catplayer"), age: 18 }];
    for messed_type in enum_vec {
        messed_type.display_type();
    }
}

#[derive(Debug)]
enum MessedType {
    //unit
    A,
    //tuple struct
    B(u32),
    //normal struct
    C {
        name: String,
        age: usize,
    },
}

impl MessedType {
    fn display_type(&self) {
        println!("the messed type is {:#?}", self)
    }
}