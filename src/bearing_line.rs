use crate::point::Point;

/// This struct defines a BearingLine, which refers to a straight line
/// that represents the direction or angle from one point to another.
pub struct BearingLine {
    pub point: Point,
    pub bearing: f64,
    pub declination: f64,
    pub true_bearing: f64,
}

impl BearingLine {
    /// Creates a new `BearingLine` without declination.
    ///
    /// This constructor is intended to be used when the declination is zero (0). It calls the secondary
    /// constructor (`with_declination`) with a declination of 0.0.
    ///
    /// # Parameters:
    /// - `latitude`: The latitude of the point, which can be an integer or a float. Should be between -90 and 90 degrees.
    /// - `longitude`: The longitude of the point, which can be an integer or a float. Should be between -180 and 180 degrees.
    /// - `bearing`: The bearing (in degrees), which can be an integer or a float. Should be between 0 and 360 degrees.
    ///
    /// # Returns:
    /// - `Ok(BearingLine)` if the input values are valid and the conversion is successful.
    /// - `Err(String)` if any of the input values are invalid or if the conversion fails.
    ///
    /// # Example:
    /// ```rust
    /// let bearing_line = BearingLine::new(37.7749, -122.4194, 45.0);
    /// assert!(bearing_line.is_ok());
    /// ```   
    pub fn new<T, U, V>(latitude: T, longitude: U, bearing: V) -> Result<Self, String>
    where
        T: TryInto<f64>,
        U: TryInto<f64>,
        V: TryInto<f64>,
    {
        Self::with_declination(latitude, longitude, bearing, 0)
    }

    /// Creates a new `BearingLine` with a specified declination.
    ///
    /// This constructor allows you to provide both a bearing and a declination (which is a correction angle
    /// applied to the bearing). The resulting true bearing is calculated as `bearing + declination`.
    ///
    /// # Parameters:
    /// - `latitude`: The latitude of the point, which can be an integer or a float. Should be between -90 and 90 degrees.
    /// - `longitude`: The longitude of the point, which can be an integer or a float. Should be between -180 and 180 degrees.
    /// - `bearing`: The bearing (in degrees), which can be an integer or a float. Should be between 0 and 360 degrees.
    /// - `declination`: The declination (in degrees), which can be an integer or a float. Should be between -180 and 180 degrees.
    ///
    /// # Returns:
    /// - `Ok(BearingLine)` if the input values are valid and the conversion is successful.
    /// - `Err(String)` if any of the input values are invalid or if the conversion fails.
    ///
    /// # Example:
    /// ```rust
    /// let bearing_line = BearingLine::with_declination(37.7749, -122.4194, 45.0, 10.0);
    /// assert!(bearing_line.is_ok());
    /// ```
    pub fn with_declination<T, U, V, W>(
        latitude: T,
        longitude: U,
        bearing: V,
        declination: W,
    ) -> Result<Self, String>
    where
        T: TryInto<f64>,
        U: TryInto<f64>,
        V: TryInto<f64>,
        W: TryInto<f64>,
    {
        let bearing = bearing
            .try_into()
            .map_err(|_| "Failed to convert bearing to f64. It should be an int or float between 0 and 360 degrees.".to_string())?;
        let declination = declination
            .try_into()
            .map_err(|_| "Failed to convert declination to f64. It should be n int or float between -180 and 180 degrees.".to_string())?;

        let point = Point::new(latitude, longitude)?;
        let true_bearing = bearing + declination;

        // Check bearing range
        if !(0.0..=360.0).contains(&bearing) {
            return Err("Bearing must be between 0 and 360 degrees.".to_string());
        }

        // Check declination range
        if !(-180.0..=180.0).contains(&declination) {
            return Err("Declination must be between -180 and 180 degrees (it should probably be a lot smaller).".to_string());
        }

        Ok(BearingLine {
            point,
            bearing,
            declination,
            true_bearing,
        })
    }

    pub fn latitude(&self) -> f64 {
        self.point.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.point.longitude
    }

    pub fn _true_bearing_to_radians(&self) -> f64 {
        self.true_bearing.to_radians()
    }

    pub fn lat_to_radians(&self) -> f64 {
        self.point.latitude.to_radians()
    }

    pub fn lon_to_radians(&self) -> f64 {
        self.point.longitude.to_radians()
    }
}

// Implement Display for custom string format
impl std::fmt::Display for BearingLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BearingLine {{ lat: {}, lon: {}, bearing: {}, declination: {} }}",
            self.latitude(),
            self.longitude(),
            self.bearing,
            self.declination
        )
    }
}
