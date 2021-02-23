use std::collections::{BTreeMap, HashMap};
use std::i32::MAX;
use std::option::Option::Some;
use std::ptr::null;

const VOWEL_WORDS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
const HAY: &str = "hay";
const AY: &str = "ay";
const CONNECT_SYMBOL: char = '-';

#[derive(Debug)]
pub struct Answer1 {
    mean: i32,
    median: i32,
    mode: i32,
}

pub fn get_answer1(sorted_list: Vec<i32>) -> Answer1 {
    let usize_length = sorted_list.len();

    if usize_length == 0 {
        panic!("empty vector!")
    };


    let mut total = 0;
    let mut length: i32 = 0;
    let mut hash_map = HashMap::new();
    for value in &sorted_list {
        total += value;
        length += 1;
        //if we can get the target count we add 1 otherwise we set value 1
        if let Some(count) = hash_map.get(value) {
            hash_map.insert(value, count + 1);
        } else {
            hash_map.insert(value, 1);
        }
    }

    let mean = total / length;

    let middle_position = match usize_length {
        1 => 0,
        _ => usize_length / 2 - 1,
    };
    let median = sorted_list[middle_position];

    let mut max_count = 0;
    let mut mode = 0;
    for (value, count) in hash_map.into_iter() {
        if count > max_count {
            max_count = count;
            mode = *value;
        }
    }

    Answer1 {
        mean,
        median,
        mode,
    }
}


pub fn convert_pig_latin(str: String) -> String {
    let mut pig_latin = String::new();
    let mut index = 0;
    let mut result = NotVowelResult {
        flag: false,
        value: ' ',
    };
    for char in str.chars() {
        if index == 0 {
            if VOWEL_WORDS.contains(&char) {
                pig_latin.push(char)
            } else {
                result = NotVowelResult {
                    flag: true,
                    value: char,
                }
            }
        } else {
            pig_latin.push(char);
        }
        index += 1;
    }

    //now we handle result
    pig_latin.push(CONNECT_SYMBOL);
    if result.flag {
        pig_latin.push(result.value);
        pig_latin.push_str(AY);
    } else {
        pig_latin.push_str(HAY);
    }

    pig_latin
}

struct NotVowelResult {
    flag: bool,
    value: char,
}
