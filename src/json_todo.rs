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
        Date::from(self.date.to_string())
    }

    pub fn parse_time(&self) -> Time {
        Time::from(self.time.to_string())
    }
}
