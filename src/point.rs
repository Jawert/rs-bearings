// A point consists of a latitude and longitude
// First a struct is defined
#[derive(Debug)]
pub struct Point {
    pub latitude: f64, // f64 (double-precision float) so it can handle floats and integers
    pub longitude: f64,
}

// Second, behavior is defined separately.
impl Point {
    pub fn new(latitude: impl Into<f64>, longitude: impl Into<f64>) -> Result<Self, String> {
        let latitude = latitude.into();
        let longitude = longitude.into();

        // Check latitude range
        if !(-90.0..=90.0).contains(&latitude) {
            return Err("Latitude must be between -90 and 90 degrees.".to_string());
        }

        // Check longitude range (assuming -180 to 180 degrees)
        if !(-180.0..=180.0).contains(&longitude) {
            return Err("Longitude must be between -180 and 180 degrees.".to_string());
        }

        Ok(Point { latitude, longitude })
    }
}
