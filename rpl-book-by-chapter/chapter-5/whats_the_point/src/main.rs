struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Calculates the distance of another point from us (self)
    fn distance(&self, other: &Point) -> f64 {
        f64::sqrt(f64::powi(other.x - self.x, 2) + f64::powi(other.y - self.y, 2))
    }

    fn distance_between_two_points(point1: &Point, point2: &Point) -> f64 {
        Point::distance(point1, point2)
    }
}

fn main() {
    println!("Hello, points! \n");

    let origin = Point {
        x: 0.0,
        y: 0.0,
    };

    let backyard_point = Point {
        x: 1.0,
        y: 0.0,
    };

    let far_out_point = Point {
        x: 1235.23,
        y: 32513.777,
    };

    let boring_point = Point {
        x: 12345.6789,
        y: 9876.54321,
    };

    assert!(origin.distance(&backyard_point) == 1.0);

    println!("Distance between origin and far out point: {:.2} meters", origin.distance(&far_out_point));

    println!("Distance between far out point and boring point: {:.2} meters", Point::distance_between_two_points(&far_out_point, &boring_point));
}
