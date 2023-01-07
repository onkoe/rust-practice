fn main() {
    let mut x = 1;
    println!("The value of x is: {}", x);

    println!("The value of x is not: {}", x + x);

    println!("The value of x can never be: {}", x/9);
    let y = true; 
    println!("The value of y is: {}", y);

    let y = false; 
    println!("The value of y is now: {}", y);

    const STRING: &str = "hello!";

    println!("{}", STRING);
}
