// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECOND: f64 = 31_557_600.0;
const MERCURY_YEAR_IN_SECOND: f64 = 0.2408467 * EARTH_YEAR_IN_SECOND;
const VENUS_YEAR_IN_SECOND: f64 = 0.61519726 * EARTH_YEAR_IN_SECOND;
const MARS_YEAR_IN_SECOND: f64 = 1.8808158 * EARTH_YEAR_IN_SECOND;
const JUPITER_YEAR_IN_SECOND: f64 = 11.862615 * EARTH_YEAR_IN_SECOND;
const SATURN_YEAR_IN_SECOND: f64 = 29.447498 * EARTH_YEAR_IN_SECOND;
const URANUS_YEAR_IN_SECOND: f64 = 84.016846 * EARTH_YEAR_IN_SECOND;
const NEPTUNE_YEAR_IN_SECOND: f64 = 164.79132 * EARTH_YEAR_IN_SECOND;

#[derive(Debug)]
pub struct Duration {
    second: f64,
}
impl Duration {
    pub fn second(&self) -> f64 {
        self.second
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { second: s as f64 }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.second() / Self::ORBITAL_PERIOD
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
    const ORBITAL_PERIOD: f64 = MERCURY_YEAR_IN_SECOND;
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = VENUS_YEAR_IN_SECOND;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = EARTH_YEAR_IN_SECOND;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = MARS_YEAR_IN_SECOND;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = JUPITER_YEAR_IN_SECOND;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = SATURN_YEAR_IN_SECOND;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = URANUS_YEAR_IN_SECOND;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = NEPTUNE_YEAR_IN_SECOND;
}
