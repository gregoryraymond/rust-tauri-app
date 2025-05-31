use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    /// Latitude in decimal degrees.
    pub latitude: f64,
    /// Longitude in decimal degrees.
    pub longitude: f64,
    /// Accuracy level of the latitude and longitude coordinates in meters.
    pub accuracy: f64,
    /// Accuracy level of the altitude coordinate in meters, if available.
    /// Available on all iOS versions and on Android 8 and above.
    pub altitude_accuracy: Option<f64>,
    /// The altitude the user is at, if available.
    pub altitude: Option<f64>,
    // The speed the user is traveling, if available.
    pub speed: Option<f64>,
    /// The heading the user is facing, if available.
    pub heading: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Creation time for these coordinates.
    // TODO: Check if we're actually losing precision.
    pub timestamp: u64,
    /// The GPS coordinates along with the accuracy of the data.
    pub coords: Coordinates,
}