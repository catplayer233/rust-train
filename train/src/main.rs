fn main() {
    mutable();

    tuple();

    array();

    function_parameter(32);

    function_4_expression();

    println!(
        "call a function which has returned a i32 integer: {}",
        function_with_return(2)
    );

    if_control_flow(32);

    if_control_flow(-32);

    if_control_flow(0);

    println!(
        "parameter is {}, and the changed value is {}",
        -32,
        if_with_return(-32)
    );

    loop_control_flow();

    let c_tem = 32.0;

    println!("c is {}, f is {}", c_tem, temperatures_convert(c_tem));

    let target_num = 32;

    println!(
        "target num is {}, and the fibonacci number is {}",
        target_num,
        fibonacci_number_generate(target_num)
    );

    ownership_explore();

    refrence_explore();

    let mut s = String::from("hello world");

    let target = str_slice(&s);

    println!("target is {}", target);
    s.clear();
}

fn mutable() {
    //in rust, the variable defaulty is imutable,
    // so if you want to reassign you should add keyword mut when you declare the variable
    let mut a = 123;
    println!("declare a is {} ", a);
    a = 34;
    println!("after reassign a is {}", a);
}

fn tuple() {
    //in rust, tuple means a set which can hold some data with different data types
    //the tuple is fixed length
    let tuple = (1, 2.9, 'A');

    let (x, y, z) = tuple;

    println!("x-> {}, y-> {}, z-> {}", x, y, z);

    println!(
        "first one is {}, second is {}, last is {}",
        tuple.0, tuple.1, tuple.2
    );

    println!("tuple is {:?}", tuple);
}

fn array() {
    //in rust, array's element must has same data type, and also be fixed-length
    //and the array stored as stack, you can use Vector instead of array when you want more fliexable usage
    let integer: usize = 10000000;

    let float = integer / 100;
    println!("float is {}", float);

    let bool_var: bool = false;

    let array = [true, false, bool_var];

    println!("array is {:?}", array);

    //target array was assigned by [n;m] means has m elements whose value all are n
    let another_array = [3; 5];

    println!("another array is {:?}", another_array);
}

//in function signature, you must decalre the type of each parameter
fn function_parameter(x: i32) {
    println!("the argument's value is {}", x)
}

fn function_4_expression() {
    let x = 3;
    let y = {
        //x->4
        let x = x + 1;
        //4*2 = 8 as the result and assign the value to y
        //expression should not add ; in the end,
        //if you add this means this is a statement, and the result
        //will not be returned
        x * 2
    };

    println!("get the expression value is: {}", y)
}

/* return value function:

fn FUNCTION_NAME(PARAMETER_NAME:TYPE...) -> RETURN_TYPE {
    ...
 } */
fn function_with_return(x: i32) -> i32 {
    x * 2
}

//when the if branches more than 2, we should use the match feature
fn if_control_flow(x: i32) {
    if x < 0 {
        println!("the parameter is less than zero")
    } else if x > 0 {
        println!("the parameter is bigger than zero")
    } else {
        println!("the parameter is equal zero")
    }
}

fn if_with_return(x: i32) -> i32 {
    let condition = x > 0;

    if condition {
        x
    } else {
        -x
    }
}

fn loop_control_flow() {
    let mut x = 0;
    let counter = loop {
        x += 1;
        if x == 10 {
            break x * 2;
        }
    };

    println!("after loop executed, the counter is {}", counter);

    x = 0;
    while x < 10 {
        x += 1;
        println!("execution not stoped, when the value is {}", x)
    }

    let array = [1, 2, 3, 4];

    for target in array.iter() {
        println!("get the element by for loop :{}", target)
    }

    for reverse in (1..4).rev() {
        println!("get the reverse element by for loop :{}", reverse)
    }
}

fn temperatures_convert(c: f64) -> f64 {
    9.0 / 5.0 * c + 32.0
}

fn fibonacci_number_generate(target_num: i32) -> i32 {
    if target_num <= 0 {
        panic!("target num :{} must more than 0", target_num)
    } else if target_num == 1 || target_num == 2 {
        target_num
    } else {
        fibonacci_number_generate(target_num - 1) + fibonacci_number_generate(target_num - 2)
    }
}

fn ownership_explore() {
    let s1 = String::from("hello");
    let s2 = s1;
    // you can not use s1 again
    //because the actual target refrence was moved to s2 and s1 no longer valid any more
    // println!("{}", s1);
    println!("{}", s2);
    //clone
    let s3 = s2.clone();
    //it's ok we use s2 again, because we call clone instead of move the orginal string's refrence
    println!("{}", s2);
    println!("{}", s3);

    let x = 5;
    let y = x;
    //it's ok when we use x, because the x is basic data type which was stored in stack,
    //when assign the x to y, rust will just copy 5 in stack and store it in stack again
    println!("{},{}", x, y);
}

fn refrence_explore() {
    let origin_let = String::from("refrence_explore");

    // let length = string_length_without_refrence(origin_let);
    //we can not do this because the origin_let's ownership was moved
    // println!("string is {}, and length is {}", origin_let, length);

    // when we user & keyword means the origin_let's ownership was brrowed to function: string_length
    //after the invoke end, the ownership will be returned to origin_let
    let length = string_length(&origin_let);
    println!("string is {}, and length is {}", origin_let, length)
}

fn string_length_without_refrence(str: String) -> usize {
    str.len()
}

fn string_length(str: &String) -> usize {
    str.len()
}

fn str_slice(target_slice: &str) -> &str {
    let bytes = target_slice.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &target_slice[..index];
        }
    }
    target_slice
}
