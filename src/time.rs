use std::fmt;

pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hour = if self.hour == 0 {
            String::from("00")
        } else {
            format!("{}", self.hour)
        };
        let minute = if self.minute == 0 {
            String::from("00")
        } else {
            format!("{}", self.minute)
        };
        write!(f, "{}:{}", hour, minute)
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
