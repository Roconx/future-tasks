use std::fmt;

pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hour, self.minute)
    }
}
