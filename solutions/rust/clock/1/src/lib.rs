use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
#[derive(Clone)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = (hours + minutes / 60) % 24;
        let mut m = minutes % 60;
        if m < 0 {
            m = m + 60;
            h -= 1;
        }
        if h < 0 {
            h += 24;
        }
        Self {
            hours: h,
            minutes: m
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let min = self.minutes + minutes;
        Self::new(self.hours, min)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter::<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, target: &Clock) -> bool {
        self.hours == target.hours &&
        self.minutes == target.minutes
    }
}