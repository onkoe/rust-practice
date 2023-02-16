use std::{io, str::FromStr};

fn main() {
    let mut inputs: (Option<f64>, Option<f64>, Option<char>) = (None, None, None);

    while inputs.0 == None {
        print!("Please provide your first number: ");
        let given_user_string: Option<String> = get_user_input();
        println!();

        inputs.0 = match given_user_string {
            Some(string) => match f64::from_str(&string.trim()) {
                Ok(number) => Some(number),
                Err(_) => None,
            },
            None => None,
        };

        println!("{:#?}", inputs);
    }

    while inputs.1 == None {
        print!("Please provide your second number: ");
        let given_user_string: Option<String> = get_user_input();
        println!();

        inputs.1 = match given_user_string {
            Some(string) => match f64::from_str(&string.trim()) {
                Ok(number) => Some(number),
                Err(_) => None,
            },
            None => None,
        };

        println!("{:#?}", inputs);
    }

    while inputs.2 == None {
        print!("Please provide your operation [+, -, x, /]: ");
        let given_user_string: Option<String> = get_user_input();
        println!();

        inputs.2 = match given_user_string {
            Some(string) => match char::from_str(&string.trim()) {
                Ok(operation) => match operation {
                    '+' | '-' | 'x' | '/' => Some(operation),
                    _ => None,
                },
                Err(_) => None,
            },
            None => None,
        };

        println!("{:#?}", inputs);
    }

    println!(
        "{:?}",
        calculate(
            unwrap_option(inputs.0),
            unwrap_option(inputs.1),
            unwrap_option(inputs.2),
        )
    );
}

fn get_user_input() -> Option<String> {
    let mut user_input_string = String::new();

    match io::stdin().read_line(&mut user_input_string) {
        Ok(_) => {
            println!("user_input_string: {}", user_input_string);
            return Some(user_input_string);
        }
        Err(_) => {
            println!("user_input_string: {}", user_input_string);
            return None;
        }
    }
}

fn unwrap_option<Data: std::fmt::Debug>(input: Option<Data>) -> Data {
    match input {
        Some(data) => return data,
        None => panic!("Data was invalid! input: {:?}", input),
    }
}

fn calculate(number_1: f64, number_2: f64, operation: char) -> Option<f64> {
    let result: Option<f64>;

    match operation {
        'x' => result = Some(number_1 * number_2),
        '/' => result = Some(number_1 / number_2),
        '+' => result = Some(number_1 + number_2),
        '-' => result = Some(number_1 - number_2),
        _ => result = None,
    }

    if result != None {
        return Some(result?);
    }
    result
}
