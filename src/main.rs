mod bearing_line;
mod geo_helpers;
mod point;
mod safe_math;

use bearing_line::BearingLine;
use geo_helpers::{get_haversine_distance, get_intersection};

fn main() {
    let bearing_line =
        BearingLine::new(37.86203, -119.43397, 122.9).expect("Failed to create bearing line.");

    println!("Latitude: {}", bearing_line.latitude());
    println!("Longitude: {}", bearing_line.longitude());
    println!("{}", bearing_line);

    let bearing_line2 = BearingLine::new(37.87366, -119.38145, 216.5)
        .expect("Failed to create bearing line with declination.");
    println!("Latitude: {}", bearing_line2.latitude());
    println!("Longitude: {}", bearing_line2.longitude());
    println!("{}", bearing_line2);

    println!(
        "Haversine Distance: {}",
        get_haversine_distance(&bearing_line, &bearing_line2)
    );
    match get_intersection(&bearing_line, &bearing_line2) {
        Ok(intersection) => {
            println!(
                "Intersection found at latitude: {}, longitude: {}",
                intersection.latitude, intersection.longitude
            );
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
