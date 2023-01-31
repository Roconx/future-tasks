mod date;
mod json_todo;
mod time;
mod time_remaining;
mod todo;
mod todos;

use crate::todos::Todos;

use std::env;

fn main() {
    let mut todos = Todos::parse();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("{}", todos);
        }
        2 => {
            let argument = &args[1];
            match argument.as_str() {
                "add" => todos.add(),
                "remove" => todos.remove(),
                _ => (),
            }
        }
        _ => (),
    }
}
