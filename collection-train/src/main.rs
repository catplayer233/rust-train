use hash_map_explorer as hash_map_mod;
use string_explorer as string_mod;
use task as task_mode;
use vector_explorer as vector_mod;

mod string_explorer;
mod vector_explorer;
mod hash_map_explorer;
mod task;

fn main() {
    let integer_vector = vec![11, 20, 30, 30, 90, 100, 120];

    println!("the target vector is {:#?}", integer_vector);
    let answer1 = task_mode::get_answer1(integer_vector);
    println!("answer1 is: {:#?}", answer1);

    let target_str = String::from("first");
    println!("the target string is {}", target_str);
    println!("converted string is {}", task_mode::convert_pig_latin(target_str));
}

fn explore() {
    vector_mod::explore();
    string_mod::explore();
    hash_map_mod::explore();
}
