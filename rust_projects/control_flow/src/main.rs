fn main() {
    if 1 == 2 {
        println!("math is broken!");
    } else {
        println!("welcome! everything is fine!");
    }

    let formal = true; 
    let greeting = if formal {
        "good evening"
    } else {
       "hey bruh"
    };

    println!("{}", greeting);

    let boolean = true; 

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{}", binary);
}
