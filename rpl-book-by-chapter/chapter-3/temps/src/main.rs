use colored::*;
use std::io;

enum TemperatureType {
    Fahreheit,
    Celsius,
}

struct Temperature {
    temp_type: TemperatureType,
    number: f32,
}

fn main() {
    println!();
    println!("{}", "Welcome to Temperature Changer!".yellow());
    println!();

    println!(
        "{}",
        "Please provide a number in degrees F or C.".bright_red()
    );
    println!(
        "{} {} {}",
        "Format it like:".red(),
        "123".bright_green(),
        "C".bright_magenta()
    );

    println!();
    let temperature = get_user_number();
    println!();

    match temperature.temp_type {
        TemperatureType::Fahreheit => println!(
            "{} F is {} in Celsius!",
            format!("{:.2}", temperature.number).red(),
            format!("{:.2}", fahrenheit_to_celsius(&temperature.number)).bright_green()
        ),
        TemperatureType::Celsius => println!(
            "{} C is {} in Fahrenheit!",
            format!("{:.2}", temperature.number).red(),
            format!("{:.2}", celsius_to_farenheit(&temperature.number)).bright_green()
        ),
    };
}

/// Converts degrees in Celsius to Fahreheit.
fn celsius_to_farenheit(input_in_celsius: &f32) -> f32 {
    (input_in_celsius * (9.0 / 5.0)) + 32.0
}

fn fahrenheit_to_celsius(input_in_fahrenheit: &f32) -> f32 {
    (input_in_fahrenheit - 32.0) * (5.0 / 9.0)
}

/// Gets a user
fn get_user_number() -> Temperature {
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input: Vec<&str> = input.split(' ').collect();
                match check_number_validity(input) {
                    None => {
                        print_invalid_output();
                        continue;
                    }
                    Some(temp) => return temp,
                };
            }
            Err(_) => {
                print_invalid_output();
                continue;
            }
        };
    }
}

fn print_invalid_output() {
    println!(
        "{} {} {}",
        "Invalid input! Please use the following format:".bright_red(),
        "45".bright_green(),
        "F".bright_magenta()
    );
}

fn check_number_validity(input: Vec<&str>) -> Option<Temperature> {
    if input.len() != 2 {
        None
    } else {
        let number_result = str::parse::<f32>(input.first()?);
        let temp_type_result = input.get(1)?.trim();

        // deal with char to c/f conversion
        let temp_type: TemperatureType = match temp_type_result {
            "c" | "C" => TemperatureType::Celsius,
            "f" | "F" => TemperatureType::Fahreheit,
            _ => return None,
        };

        // check if we got a number
        let number: f32 = match number_result {
            Ok(num) => num,
            Err(_) => return None,
        };
        Some(Temperature { temp_type, number })
    }
}
