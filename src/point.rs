use std::fmt;

/// This struct defines a coordinate which consists of two numbers, the latitude and longitude.
///
/// # Fields
/// - `latitude`: The latitude or "Y" coordinate.
/// - `longitude`: The longitude or "X" coordinate.
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

/// Creates a new `Point` with the specified latitude and longitude.
///
/// # Parameters:
/// - `latitude`: The latitude of the point, which can be provided as an integer or a float.
///   Must be in the range of -90 to 90 degrees (inclusive).
/// - `longitude`: The longitude of the point, which can be provided as an integer or a float.
///   Must be in the range of -180 to 180 degrees (inclusive).
///
/// # Returns:
/// - `Ok(Point)` if the latitude and longitude are within the valid range and can be successfully converted to `f64`.
/// - `Err(String)` if either the latitude or longitude is out of range, or if the conversion to `f64` fails.
///
/// # Example:
/// ```rust
/// let point = Point::new(37.7749, -122.4194);
/// assert!(point.is_ok());
/// let point = Point::new(100.0, -200.0);
/// assert!(point.is_err());
/// ```
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
