use std::fmt;

use chrono::{Datelike, NaiveDate};

pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.day, self.month, self.year)
    }
}

impl From<NaiveDate> for Date {
    fn from(naive_date: NaiveDate) -> Date {
        Date {
            year: naive_date.year(),
            month: naive_date.month(),
            day: naive_date.day(),
        }
    }
}

impl From<String> for Date {
    fn from(date: String) -> Date {
        let split = date.split("/").collect::<Vec<&str>>(); // "dd", "mm", "yyyy"
        Date {
            year: split[2].parse::<i32>().unwrap(),
            month: split[1].parse::<u32>().unwrap(),
            day: split[0].parse::<u32>().unwrap(),
        }
    }
}
