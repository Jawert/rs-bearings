use std::fmt;

///
/// This struct defines a coordinate which consists of two numbers, the latitude and longitude.
///
/// # Fields
/// - `latitude`: The latitude or "Y" coordinate.
/// - `longitude`: The longitude or "X" coordinate.
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    pub fn new<T, U>(latitude: T, longitude: U) -> Result<Self, String>
    where
        T: TryInto<f64>,
        U: TryInto<f64>,
    {
        let latitude = latitude
            .try_into()
            .map_err(|_| "Failed to convert latitude to f64. It should be an int or float between -90 and 90 degrees.".to_string())?;
        let longitude = longitude
            .try_into()
            .map_err(|_| "Failed to convert longitude to f64. It should be an int or float between -180 an 180 degrees.".to_string())?;

        // Check latitude range
        if !(-90.0..=90.0).contains(&latitude) {
            return Err("Latitude must be between -90 and 90 degrees.".to_string());
        }
        // Check longitude range (assuming -180 to 180 degrees)
        if !(-180.0..=180.0).contains(&longitude) {
            return Err("Longitude must be between -180 and 180 degrees.".to_string());
        }

        Ok(Point {
            latitude,
            longitude,
        })
    }
}

// Implement Display for custom string format
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}
