mod point;

use point::Point;

fn main() {
    match Point::new(51.5074, -0.1234) {
        Ok(point) => println!("{}", point),
        Err(e) => println!("Error creating point: {}", e),
    }
}

