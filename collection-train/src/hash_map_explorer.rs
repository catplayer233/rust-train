use std::collections::HashMap;

pub fn explore() {
    //create
    let mut hash_map = HashMap::new();
    //add entry
    hash_map.insert(String::from("Blue"), 10);
    hash_map.insert(String::from("Yellow"), 20);
    //zip operate
    let color_vec = vec![String::from("Black"), String::from("White")];
    let score_vec = vec![10, 30];
    //pari the key vector and the value vector and build a HashMap
    let mut hash_map: HashMap<_, _> = color_vec.into_iter().zip(score_vec).collect();

    println!("map: {:?}", hash_map);

    let green = String::from("Green");
    let green_score = 40;
    //insert will take the key and value's ownership
    //so you can not use green again, but the green_score is an integer which store in stack and just copied
    //so you can continue use the green_score
    hash_map.insert(green, green_score);
    //can not use this!
    // println!("{}", green);
    //you still can use the integer value
    println!("{}", green_score);

    //get operation
    let black_key = String::from("Black");
    display_entry(&hash_map, &black_key);

    //update operation
    //just user insert
    hash_map.insert(black_key, 90);
    display_entry(&hash_map, "Black");

    let red_reference = "red";
    //insert if the key is not existed
    hash_map.entry(String::from(red_reference)).or_insert(100);
    display_entry(&hash_map, red_reference);

    //update the old value
    *(hash_map.get_mut(red_reference).unwrap()) += 1;
    display_entry(&hash_map, red_reference);
}

fn display_entry(hash_map: &HashMap<String, i32>, key: &str) {
    let option_score = hash_map.get(key);
    println!("key:{}, value:{}", key, match option_score {
        Some(score) => score,
        None => &-1
    });
}