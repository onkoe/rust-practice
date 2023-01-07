struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

struct Point2D(u32, u32);

fn main() {
    let guy = Person {
        name:String::from("Adam"),
        age: 23,
        likes_oranges: true,
    };

    let point = Point2D(2, 4);

    if guy.age == 23 {
        println!("mf is 23");
    } else {
        println!("mf aint shit");
    }

    println!("dudes name is: {}", guy.name);

    println!("bruh this point is at {}, {}", point.0, point.1);

    println!("bruh this point is at {:?}, {:?}", point.0, point.1);


    let Point2D(x, y) = point;

    println!("point contains {:?}, {:?}!", x, y);
}

