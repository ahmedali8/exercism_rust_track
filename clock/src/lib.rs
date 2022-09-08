use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hrs: i32 = minutes / 60;
        let mut mins: i32 = minutes % 60;

        hrs = (hours + hrs) % 24;

        // negative validation
        if mins < 0 {
            hrs -= 1;
            mins += 60;
        }
        if hrs < 0 {
            hrs += 24
        }

        Clock {
            hours: hrs,
            minutes: mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
