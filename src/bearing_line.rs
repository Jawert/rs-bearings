use crate::point::Point;

static RADIUS_OF_EARTH_IN_KM: i32 = 6371;

/// This struct defines a BearingLine, which refers to a straight line
/// that represents the direction or angle from one point to another.
pub struct BearingLine {
    pub point: Point,
    pub bearing: f64,
    pub declination: f64,
    pub true_bearing: f64,
}

impl BearingLine {
    // Main constructor without declination
    pub fn new<T, U, V>(latitude: T, longitude: U, bearing: V) -> Result<Self, String>
    where
        T: TryInto<f64>,
        U: TryInto<f64>,
        V: TryInto<f64>,
    {
        Self::with_declination(latitude, longitude, bearing, 0)
    }

    // Secondary constructor with declination
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

    pub fn true_bearing_to_radians(&self) -> f64 {
        self.true_bearing.to_radians()
    }

    pub fn lat_to_radians(&self) -> f64 {
        self.point.latitude.to_radians()
    }

    pub fn lon_to_radians(&self) -> f64 {
        self.point.longitude.to_radians()
    }
}

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
