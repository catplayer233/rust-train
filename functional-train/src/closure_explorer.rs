use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


pub fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushes",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} steps",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes",
                     simulated_expensive_calculation(random_number)
            );
        }
    }
}

pub fn generate_workout_closure(intensity: u32, random_num: u32) {
    //you can treat a function as a variable,
    //so that the closure function will only be valid in this function scope
    //between || is the arguments: separate them with commons, after that with code block
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushes", expensive_closure(intensity));
        println!("Next, do {} steps", expensive_closure(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_closure(random_num));
        }
    }
}

pub fn generate_workout_closure_trait(intensity: u32, random_num: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushes", cacher.value(intensity));
        println!("Next, do {} steps", cacher.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", cacher.value(random_num));
        }
    }
}

//if we do not specify the arguments and the return type, we should keep the closure be invoked
//by the same type arguments
// fn wrong_closure() {
//     let wrong_closure = |x| x;
//     let int_value = wrong_closure(1);
//     let string_value = wrong_closure(String::from("test"));
// }

struct Cacher<T>
//T bound a function which has a parameter with u32 type and return a u32 value
//Fn(u32)->u32
    where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(cache_value) => cache_value,
            None => {
                let result = (self.calculation)(arg);
                self.value = Some(result);
                result
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cacher() {
        let mut cacher = Cacher::new(|x| x);
        let value_1 = cacher.value(1);
        let value_2 = cacher.value(2);
        assert_eq!(value_2, 2);
    }
}
