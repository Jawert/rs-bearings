mod bearing_line;
mod point;

use bearing_line::BearingLine;

fn main() {
    let bearing_line = BearingLine::new(37.7749, -122.4194);

    println!("Latitude: {}", bearing_line.latitude());
    println!("Longitude: {}", bearing_line.longitude());
    println!("{}", bearing_line);
}
