use panic_explorer::explore as panic_explore;
use result_explorer::explore as result_explore;

mod panic_explorer;
mod result_explorer;

fn main() {
    // panic_explore();
    // result_explorer::explore();
    let file_str = result_explorer::read_explore().expect("can not read, reason: ");
    println!("get the result: {}", file_str);
}
