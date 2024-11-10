use crate::point::Point;
use std::fmt;
///
/// This struct defines a BearingLine, which refers to a straight line
/// that represents the direction or angle from one point to another.
///
pub struct BearingLine {
    pub point: Point,
}

impl BearingLine {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        let point = Point::new(latitude, longitude).expect("Failed to create Point");
        BearingLine { point }
    }

    pub fn latitude(&self) -> f64 {
        self.point.latitude
    }
    pub fn longitude(&self) -> f64 {
        self.point.longitude
    }
}

impl fmt::Display for BearingLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bearing Line: Point at ({}, {})",
            self.point.latitude, self.point.longitude
        )
    }
}
