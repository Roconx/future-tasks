use crate::date::Date;
use crate::json_todo::JsonTodo;
use crate::parser::get_input;
use crate::time::Time;
use crate::time_remaining::TimeRemaining;

use chrono::{offset::Local, DateTime, TimeZone};
use std::cmp::Ordering;
use std::fmt;
use std::time::SystemTime;

pub struct Todo {
    pub title: String,
    pub description: String,
    pub topic: String,
    pub date: Date, // dd/mm/yyyy
    pub time: Time, // hh:mm
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {} \n\nDescription: {} \n\nTopic: {} \n\nDue date: {} at {} \n\nTime reamaining: {}\n\n
---------------------------------------------------------------------------------------------------------\n\n",
         self.title, self.description, self.topic, self.date, self.time, self.time_left())
    }
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_left().cmp(&other.time_left())
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for Todo {}

impl Todo {
    pub fn time_left(&self) -> TimeRemaining {
        let current_time = SystemTime::now();
        let date_time: DateTime<Local> = current_time.into();
        let day: DateTime<Local> = Local
            .with_ymd_and_hms(
                self.date.year,
                self.date.month,
                self.date.day,
                self.time.hour,
                self.time.minute,
                0,
            )
            .unwrap();

        let time_remaining = day.signed_duration_since(date_time);

        let days_left = time_remaining.num_days() as i32;
        let hours_left = time_remaining.num_hours() as i32 % 24;
        let minutes_left = time_remaining.num_minutes() as i32 % 60;

        TimeRemaining {
            days: days_left,
            hours: hours_left,
            minutes: minutes_left,
        }
    }

    pub fn to_json_todo(&self) -> JsonTodo {
        JsonTodo {
            title: self.title.to_owned(),
            description: self.description.to_owned(),
            topic: self.topic.to_owned(),
            date: format!("{}", self.date),
            time: format!("{}", self.time),
        }
    }

    pub fn new() -> Todo {
        let todo_json = JsonTodo {
            title: get_input("Enter the title: "),
            description: get_input("Enter the description: "),
            topic: get_input("Enter the topic: "),
            date: get_input("Enter the date: "),
            time: get_input("Enter the time: "),
        };

        Todo {
            title: todo_json.title.to_owned(),
            description: todo_json.description.to_owned(),
            topic: todo_json.topic.to_owned(),
            date: todo_json.parse_date(),
            time: todo_json.parse_time(),
        }
    }
}
