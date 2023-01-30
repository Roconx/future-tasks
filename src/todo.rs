use crate::parser::{TodoJson, get_input};
use chrono::{offset::Local, DateTime, TimeZone};
use std::fmt::{self, Debug};
use std::time::SystemTime;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hour, self.minute)
    }
}

pub struct TimeRemaining {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
}

impl fmt::Display for TimeRemaining {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let days = if self.days != 1 { "days" } else { "day" };
        let hours = if self.hours != 1 { "hours" } else { "hour" };
        let minutes = if self.minutes != 1 {
            "minutes"
        } else {
            "minute"
        };

        write!(
            f,
            "{} {}, {} {}, {} {}",
            self.days, days, self.hours, hours, self.minutes, minutes
        )
    }
}

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

    pub fn to_json_todo(&self) -> TodoJson {
        TodoJson { title: self.title.to_owned(),  description: self.description.to_owned(), topic: self.topic.to_owned(), date: format!("{}", self.date), time: format!("{}", self.time) }
    }
    
    pub fn new() -> Todo {
        let todo_json = TodoJson {
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
