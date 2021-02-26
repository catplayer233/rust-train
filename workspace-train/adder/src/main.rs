use add_one;

fn main() {
    let num = 10;
    println!("target: {} and invoke add one: {}",
             num, add_one::add_one(num));
}
