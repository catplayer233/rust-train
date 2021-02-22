fn main() {
    let write_message = Message::Write(String::from("hello"));
    write_message.call();

    let message = get_message(3);
    message.expect("no such code for Message");

    let value = Some(99);
    let plus_value = plus_one(value);
    //so enum like the primary data type, just copy not take the ownership
    println!("value is {}", value.expect("get value"));
    println!("plus_value is {}", plus_value.expect("there is null"));

    if_let_synax(Some(7));
}

//in rust, the enum element can have different struct
//this is not as same as java or other language
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("this function is a message's function")
    }
}

fn get_message(code: u32) -> Option<Message> {
    match code {
        0 => Some(Message::Quit),
        1 => Some(Message::Move { x: 2, y: 2 }),
        2 => Some(Message::Write(String::from("hello"))),
        3 => Some(Message::ChangeColor(1, 2, 3)),
        //"_" means anything else, like java switch control flow's default keyword
        _ => None,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(some: Option<i32>) -> Option<i32> {
    match some {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn if_let_synax(value: Option<u32>) {
    if let Some(3) = value {
        println!("the exact value: {}", 3)
    } else {
        println!("the value is not as same as target: {}", 3)
    }
}
