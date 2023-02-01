use crate::date::Date;
use crate::json_todo::JsonTodo;
use crate::time::Time;
use crate::time_remaining::TimeRemaining;
use crate::todos::Todos;

use chrono::{offset::Local, DateTime, TimeZone};
use colored::*;
use inquire::{CustomUserError, DateSelect, Text};
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
        write!(f, "{} {} \n\n{} {} \n\n{} {} \n\n{} {} {} {} \n\n{} {}\n\n
---------------------------------------------------------------------------------------------------------\n\n",
         "Title:".green(), self.title, "Description:".green(), self.description, "Topic:".green(), self.topic, "Due date:".green(), self.date, "at".green(), self.time, "Time reamaining:".green(), self.time_left())
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
        let title = Text::new("Enter the title: ").prompt().unwrap();
        let description = Text::new("Enter the description: ").prompt().unwrap();
        let topic = Text::new("Enter the topic: ")
            .with_autocomplete(&suggester)
            .prompt()
            .unwrap();
        let date = DateSelect::new("Enter the date: ").prompt().unwrap();
        let time = Text::new("Enter the time: ")
            .with_default("23:59")
            .prompt()
            .unwrap();

        Todo {
            title: title.to_owned(),
            description: description.to_owned(),
            topic: topic.to_owned(),
            date: Date::from(date),
            time: Time::from(time),
        }
    }

    pub fn edit(&mut self) {
        self.title = Text::new("Enter the title: ")
            .with_default(&self.title)
            .prompt()
            .unwrap();
        self.description = Text::new("Enter the description: ")
            .with_default(&self.description)
            .prompt()
            .unwrap();
        self.topic = Text::new("Enter the topic: ")
            .with_default(&self.topic)
            .with_autocomplete(&suggester)
            .prompt()
            .unwrap();
        self.date = DateSelect::new("Enter the date: ")
            .with_default(self.date.into())
            .prompt()
            .unwrap()
            .into();
        self.time = Text::new("Enter the time: ")
            .with_default(format!("{}", self.time).as_str())
            .prompt()
            .unwrap()
            .into();
    }

    pub fn is_late(&self) -> bool {
        self.time_left().is_late()
    }
}

fn suggester(val: &str) -> Result<Vec<String>, CustomUserError> {
    let suggestions = Todos::get_topics();

    let val_lower = val.to_lowercase();

    Ok(suggestions
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(s))
        .collect())
}
