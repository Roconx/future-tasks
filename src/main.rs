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
                "late" => todos.late(),
                "topic" => todos.filter_by_topic(),
                "update" => todos.update(),
                _ => help(),
            }
        }
        _ => (),
    }
}

fn help() {
    println!(
        "Unknow argument, the avaliable argument are:
    add: Add a new todo
    remove: Removes a todo
    late: Filters by late todo
    topic: Filters by topic
    update: Update an existing todo"
    );
}
