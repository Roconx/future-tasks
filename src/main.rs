mod date;
mod json_todo;
mod parser;
mod time;
mod time_remaining;
mod todo;

use crate::todo::Todo;

use parser::{get_input, save_todo};
use std::env;

fn main() {
    let mut todos = parser::parse_todo();

    let args: Vec<String> = env::args().collect();

    // ./nom_programa

    match args.len() {
        1 => {
            for todo in todos {
                println!("{}", todo);
            }
        }
        2 => {
            let argument = &args[1];
            match argument.as_str() {
                "add" => {
                    let todo = Todo::new();
                    todos.push(todo);
                    save_todo(&mut todos);
                }
                "remove" => {
                    let title_to_remove = get_input("Enter the title to remove: ");
                    let mut index_to_remove: i32 = -1;
                    for (i, todo) in todos.iter().enumerate() {
                        if todo.title == title_to_remove {
                            index_to_remove = i as i32;
                        }
                    }

                    if index_to_remove == -1 {
                        println!("No matching title was found!");
                    } else {
                        todos.remove(index_to_remove as usize);
                        save_todo(&mut todos);
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }
}

