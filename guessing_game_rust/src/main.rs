use rand::prelude::*;
use std::io::*;

fn main() {
    println!("Hey, welcome to the number guessing game! Type 'q' to exit!");
    let random_guess = generate_random_number(0, 4);

    loop {
        let user_input: String = get_user_input();

        // quit if the user types 'q'
        if user_input.trim() == "q" {
            break;
        }

        // otherwise, treat user input as a guess
        if user_input.trim() == random_guess.to_string() {
            println!("Yay, you won! The number was {}!", random_guess.to_string());
            break;
        } else {
            println!("Wrong guess. Guess again!");
        }
    }
}

fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(min..max);
    random_number
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("We were unable to read the previous line. Please try again!");
    user_input.trim_end().to_string()
}
