use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

impl Circle {

    /// Constructs a new circle with radius 1.0
    fn new() -> Circle {
        Circle {radius: 1.0}
    }

    /// Changes a circle's radius to be the given radius
    fn set_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
    }

    /// Gets this circle's area
    fn area(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    /// Gets any circle's area
    fn get_area_of_circle(circle: &Circle) -> f64 {
        circle.area()
    }
}
fn main() {
    println!("Hello, world!");

    let lil_guy = Circle::new();
    assert_eq!(lil_guy.radius, 1.0);
    println!("lil_guy's area: {:.2} meters :)", lil_guy.area());
    println!();

    let mut big_guy = Circle::new();
    big_guy.set_radius(47.123);
    println!("Woah! big_guy's area is {:.2} meters?!?!??!", Circle::get_area_of_circle(&big_guy));
}