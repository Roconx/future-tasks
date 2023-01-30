use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self, Write};

use std::fs;

use crate::todo::{Date, Time, Todo};

#[derive(Deserialize, Serialize)]
pub struct TodoJson {
    pub title: String,
    pub description: String,
    pub topic: String,
    pub date: String,
    pub time: String,
}

#[derive(Deserialize, Serialize)]
pub struct TodoVector {
    pub todo: Vec<TodoJson>,
}

impl TodoJson {
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

pub fn parse_todo() -> Vec<Todo> {
    let todo_json = fs::read_to_string("todo.json").unwrap();

    let json_todos: TodoVector = serde_json::from_str(todo_json.as_str()).unwrap();

    let mut todos: Vec<Todo> = Vec::new();

    for json_todo in json_todos.todo {
        let todo = Todo {
            title: json_todo.title.to_owned(),
            description: json_todo.description.to_owned(),
            topic: json_todo.topic.to_owned(),
            date: json_todo.parse_date(),
            time: json_todo.parse_time(),
        };
        todos.push(todo);
    }

    todos
}

pub fn save_todo(todos: &mut Vec<Todo>) {
    todos.sort();

    let mut todo_vector = TodoVector { todo: Vec::new() };

    for todo in todos {
        todo_vector.todo.push(todo.to_json_todo());
    }

    let json_string = serde_json::to_string_pretty(&todo_vector).unwrap();

    fs::write("todo.json", json_string).unwrap();
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();

    String::from(input.trim())
}

