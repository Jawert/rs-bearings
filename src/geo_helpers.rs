use crate::bearing_line::BearingLine;
use crate::point::Point;

static RADIUS_OF_EARTH_IN_KM: f64 = 6371.0;

pub trait HasPoint {
    fn point(&self) -> &Point;
}

impl HasPoint for Point {
    fn point(&self) -> &Point {
        self
    }
}

impl HasPoint for BearingLine {
    fn point(&self) -> &Point {
        &self.point
    }
}

pub fn get_haversine_distance<T: HasPoint>(p1: &T, p2: &T) -> f64 {
    let point1 = p1.point();
    let point2 = p2.point();

    // Use the haversine formula to calculate the distance between the points
    let lat1 = point1.latitude.to_radians();
    let lon1 = point1.longitude.to_radians();
    let lat2 = point2.latitude.to_radians();
    let lon2 = point2.longitude.to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    RADIUS_OF_EARTH_IN_KM * c // Return the distance
}
