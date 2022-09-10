const SECONDS_IN_YEAR: f64 = 31557600 as f64;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet_macro {
    // match like arm for macro
    // ident: identifier, expr: expression
    ($planet_name:ident, $divisor: expr) => {
        // macro expands to this code:
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / SECONDS_IN_YEAR / $divisor
            }
        }
    };
}

planet_macro!(Mercury, 0.2408467);
planet_macro!(Venus, 0.61519726);
planet_macro!(Earth, 1.0);
planet_macro!(Mars, 1.8808158);
planet_macro!(Jupiter, 11.862615);
planet_macro!(Saturn, 29.447498);
planet_macro!(Uranus, 84.016846);
planet_macro!(Neptune, 164.79132);
