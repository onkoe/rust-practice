use std::fs::File;
use std::vec;

fn main() {
    /*
    let v = vec![0, 1, 2, 3];
    println!("{:#?}", v[2]); //replace v[2] with v[6] to get a panic!

    let fruits = vec!["banana", "apple", "coconut"];

    let first = fruits.get(0);
    println!("{:?}", first);

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_exist = fruits.get(99);
    println!("{:?}", non_exist); // shows the option enum
    */

    let f = File::open("hello.txt").expect("couldn't open the file!");

    /* // match statement version of unwrap/expect
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Couldn't open the file: {:?}", error),
    };
    */


}
