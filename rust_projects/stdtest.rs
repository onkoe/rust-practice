use std::io;
use std::fs::File;

fn main() {
    // Read data from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("You typed: {}", input);

    // Write data to standard output
    print!("Hello, world!");

    // Write data to standard error
    eprint!("This is an error message!");

    // Read data from a file
    let mut file = File::open("my_file.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents: {}", contents);

    // Write data to a file
    let mut file = File::create("my_file.txt").unwrap();
    file.write(b"Hello, file!").unwrap();

    // Copy data from one input stream to an output stream
    let mut input_file = File::open("input.txt").unwrap();
    let mut output_file = File::create("output.txt").unwrap();
    io::copy(&mut input_file, &mut output_file).unwrap();

    // Use a BufReader to read data from a file more efficiently
    let file = File::open("my_file.txt").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    println!("File contents: {}", contents);

    // Use a BufWriter to write data to a file more efficiently
    let file = File::create("my_file.txt").unwrap();
    let mut writer = io::BufWriter::new(file);
    writer.write(b"Hello, file!").unwrap();
}
