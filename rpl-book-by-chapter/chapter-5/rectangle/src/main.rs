struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn find_rectangles_area(some_rect: Rectangle) {
        println!("The rectangle you gave me has an area of {} meters!", some_rect.area());
    }
}

fn main() {
    let lil_guy: Rectangle = Rectangle {
        width: 1,
        height: 1,
    };

    let big_dude = Rectangle {
        width: 23523,
        height: 124,
    };

    assert_eq!(lil_guy.area(), 1);
    assert!(big_dude.area() > 23523);

    Rectangle::find_rectangles_area(big_dude);
    Rectangle::find_rectangles_area(lil_guy);
}
