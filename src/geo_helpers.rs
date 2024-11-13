use std::fmt;

use crate::bearing_line::BearingLine;
use crate::point::Point;
use crate::safe_math::*;

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

pub fn get_intersection(bl1: &BearingLine, bl2: &BearingLine) -> Result<Point, String> {
    let lat1 = bl1.latitude();
    let lon1 = -bl1.longitude();
    let tbr1 = bl1.true_bearing;
    let lat2 = bl2.latitude();
    let lon2 = -bl2.longitude();
    let tbr2 = bl2.true_bearing;

    if lat1 == lat2 && lon1 == lon2 {
        return Err(
            "Latitudes and Longitudes are the same! You are already at the intersection."
                .to_string(),
        );
    }

    // dst12 == distance between points 1 and 2
    let dst12 = 2.0
        * asin_safe(
            (((lat1 - lat2) / 2.0).sin().powf(2.0)
                + lat1.cos() * lat2.cos() * ((lon1 - lon2) / 2.0).sin().powf(2.0))
            .sqrt(),
        );

    // crs == course
    let base_crs12 =
        acos_safe((lat2.sin() - lat1.sin() * dst12.cos()) / (dst12.sin() * lat1.cos()));
    let base_crs21 =
        acos_safe((lat1.sin() - lat2.sin() * dst12.cos()) / (dst12.sin() * lat2.cos()));

    let crs12 = (2.0 * std::f64::consts::PI + base_crs12.copysign((lon2 - lon1).sin()))
        % (2.0 * std::f64::consts::PI);
    let crs21 = (2.0 * std::f64::consts::PI - base_crs21.copysign((lon2 - lon1).sin()))
        % (2.0 * std::f64::consts::PI);

    // find the angles between the line between our bearing lines and the line between the input points
    let mut ang1 = euclidean_modulo(
        tbr2 - tbr1 + std::f64::consts::PI,
        2. * std::f64::consts::PI,
    ) - std::f64::consts::PI;

    let mut ang2 = euclidean_modulo(
        tbr1 - tbr2 + std::f64::consts::PI,
        2. * std::f64::consts::PI,
    ) - std::f64::consts::PI;

    if ang1.sin() == 0.0 && ang2.sin() == 0.0 {
        return Err("Infinite intersections. Are the lines on top of each other?".to_string());
    } else {
        let ang1 = ang1.abs();
        let ang2 = ang2.abs();
        let ang3 = acos_safe(-ang1.cos() * ang2.cos() + ang1.sin() * ang2.sin() * dst12.cos());

        // using the values so far calculated, determine latitude and longitude of intersection
        //  dst13=math.atan2(math.sin(dst12)*math.sin(ang1)*math.sin(ang2),math.cos(ang2)+math.cos(ang1)*math.cos(ang3))
        let dst13 = f64::atan2(
            dst12.sin() * ang1.sin() * ang2.sin(),
            ang2.cos() + ang1.cos() * ang3.cos(),
        );
        //  lat3=asin_safe(math.sin(lat1)*math.cos(dst13)+math.cos(lat1)*math.sin(dst13)*math.cos(crs13))
        let lat3 = asin_safe(lat1.sin() * dst13.cos() + lat1.cos() * dst13.sin() * tbr1.cos());
        //  dlon=math.atan2(math.sin(crs13)*math.sin(dst13)*math.cos(lat1),math.cos(dst13)-math.sin(lat1)*math.sin(lat3))
        let dlon = f64::atan2(
            tbr1.sin() * dst13.sin() * lat1.cos(),
            dst13.cos() - lat1.sin() * lat3.sin(),
        );
        //  lon3=euclidean_modulo(lon1-dlon+math.pi,2*math.pi)-math.pi
        let lon3 = euclidean_modulo(
            lon1 - dlon + std::f64::consts::PI,
            2.0 * std::f64::consts::PI,
        ) - std::f64::consts::PI;
        //
        //  this formula was designed with west longitudes being positive (ugh), so flip the sign at output
        //  intersect = (round(math.degrees(lat3), 5), round(math.degrees(-lon3), 5))

        Point::new(
            (lat3.to_degrees()).round() * 100000.0 / 100000.0,
            (-lon3.to_degrees()).round() * 100000.0 / 100000.0,
        )
    }
}
