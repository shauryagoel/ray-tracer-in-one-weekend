use rand::Rng;
use std::f64::consts::PI;

/// Convert degress to radians
pub fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}

/// Generate a random f64 number in the range [min, max)
pub fn random_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng(); // TODO: see if this is needed as it may slow us down
    let random_float: f64 = rng.random();
    min + (max - min) * random_float
}
