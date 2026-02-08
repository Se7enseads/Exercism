use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn normalize(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let total_minutes = total_minutes.rem_euclid(24 * 60);

        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::normalize(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::normalize(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
