mod bearing_line;
mod geo_helpers;
mod point;

use bearing_line::BearingLine;
use geo_helpers::get_haversine_distance;

fn main() {
    let bearing_line =
        BearingLine::new(37.7749, -122.4194, 45.0).expect("Failed to create bearing line.");

    println!("Latitude: {}", bearing_line.latitude());
    println!("Longitude: {}", bearing_line.longitude());
    println!("{}", bearing_line);

    let bearing_line2 = BearingLine::with_declination(78.979, 100, 34.1, 10)
        .expect("Failed to create bearing line with declination.");
    println!("Latitude: {}", bearing_line2.latitude());
    println!("Longitude: {}", bearing_line2.longitude());
    println!("{}", bearing_line2);

    println!(
        "Haversine Distance: {}",
        get_haversine_distance(&bearing_line, &bearing_line2)
    )
}
