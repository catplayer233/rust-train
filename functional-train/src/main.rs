use closure_explorer::generate_workout;

use crate::closure_explorer::{generate_workout_closure, generate_workout_closure_trait};

mod closure_explorer;
mod iterator_explorer;

fn main() {
    let user_specific_intensity = 10;
    let random_number = 7;

    generate_workout(user_specific_intensity, random_number);
    generate_workout_closure(user_specific_intensity, random_number);
    generate_workout_closure_trait(user_specific_intensity, random_number);
}
