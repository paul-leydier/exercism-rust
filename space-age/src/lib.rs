// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { s }
    }
}

pub trait Planet {
    const YEAR_LENGTH: f64;
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEAR_DURATION: f64 = 31_557_600.0;
        d.s as f64 / (Self::YEAR_LENGTH * EARTH_YEAR_DURATION)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const YEAR_LENGTH: f64 = 0.2408467;
}
impl Planet for Venus {
    const YEAR_LENGTH: f64 = 0.61519726;
}
impl Planet for Earth {
    const YEAR_LENGTH: f64 = 1.0;
}
impl Planet for Mars {
    const YEAR_LENGTH: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const YEAR_LENGTH: f64 = 11.862615;
}
impl Planet for Saturn {
    const YEAR_LENGTH: f64 = 29.447498;
}
impl Planet for Uranus {
    const YEAR_LENGTH: f64 = 84.016846;
}
impl Planet for Neptune {
    const YEAR_LENGTH: f64 = 167.79132;
}
