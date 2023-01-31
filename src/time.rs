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

impl From<String> for Time {
    fn from(time: String) -> Time {
        let split = time.split(":").collect::<Vec<&str>>();
        Time {
            hour: split[0].parse::<u32>().unwrap(),
            minute: split[1].parse::<u32>().unwrap(),
        }
    }
}
