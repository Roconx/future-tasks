use crate::date::Date;
use crate::time::Time;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TodoVector {
    pub todo: Vec<JsonTodo>,
}

#[derive(Deserialize, Serialize)]
pub struct JsonTodo {
    pub title: String,
    pub description: String,
    pub topic: String,
    pub date: String,
    pub time: String,
}

impl JsonTodo {
    pub fn parse_date(&self) -> Date {
        let split = self.date.split("/").collect::<Vec<&str>>(); // "dd", "mm", "yyyy"
        Date {
            year: split[2].parse::<i32>().unwrap(),
            month: split[1].parse::<u32>().unwrap(),
            day: split[0].parse::<u32>().unwrap(),
        }
    }

    pub fn parse_time(&self) -> Time {
        let split = self.time.split(":").collect::<Vec<&str>>();
        Time {
            hour: split[0].parse::<u32>().unwrap(),
            minute: split[1].parse::<u32>().unwrap(),
        }
    }
}
