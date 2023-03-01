mod date;
mod json_todo;
mod time;
mod time_remaining;
mod todo;
mod todos;
mod topic;

use crate::todos::Todos;

use eframe::egui;
use std::env;

fn main() {
    let mut todos = Todos::default();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let options = eframe::NativeOptions {
                initial_window_size: Some(egui::vec2(360.0, 500.0)),
                ..Default::default()
            };
            eframe::run_native(
                "Future Tasks",
                options,
                Box::new(|_cc| Box::new(Todos::default())),
            )
            .unwrap();

            // println!("{}", todos);
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
