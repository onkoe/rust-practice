use clap::*;
use num_bigint::*;

fn main() {
    let arguments = Command::new("Fibonacci Finder")
        .version(crate_version!())
        .about("Finds fibonacci number from input")
        .author("me :)")
        .arg(arg!(<NUMBER> "the fibonacci index you want to find").value_parser(value_parser!(u64)))
        .get_matches();

    let index: u64 = *arguments.get_one("NUMBER").unwrap();

    let number = match index {
        0 => 0.to_biguint().unwrap(),
        _ => find_fibonacci(index),
    };

    println!("I think the number is... {number}..!");
}

/// Finds the fibonacci number using the fun method (brute force)
/// TODO: add threading to this muuuuuch later
fn find_fibonacci(index: u64) -> BigUint {
    let mut found: (BigUint, BigUint) = (0.to_biguint().unwrap(), 1.to_biguint().unwrap());

    for _value in 2..=index {
        dbg!(_value);
        found = (found.1.clone(), (found.0 + found.1));
    }

    found.1
}
