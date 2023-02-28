use crate::json_todo::TodoVector;
use crate::todo::Todo;
use crate::topic::Topic;
use eframe::egui::RichText;
use inquire::Select;
use std::time::Duration;

use serde_json;
use std::fmt;
use std::fs;

pub struct Todos {
    pub todos: Vec<Todo>,
    pub selected_topic: Topic,
}

impl Todos {
    pub fn parse() -> Vec<Todo> {
        let todo_json = fs::read_to_string("todo.json").unwrap();

        let json_todos: TodoVector = serde_json::from_str(todo_json.as_str()).unwrap();

        let mut todos: Vec<Todo> = Vec::new();

        for json_todo in json_todos.todo {
            let todo = Todo {
                title: json_todo.title.to_string(),
                description: json_todo.description.to_string(),
                topic: json_todo.topic.to_string(),
                date: json_todo.parse_date(),
                time: json_todo.parse_time(),
            };
            todos.push(todo);
        }

        todos
    }

    pub fn add(&mut self) {
        let todo = Todo::new();
        self.todos.push(todo);
        self.save();
    }

    pub fn remove(&mut self) {
        let options = self.get_titles();
        let title_to_remove = Select::new("Enter the title to remove: ", options).prompt();
        match title_to_remove {
            Ok(title_to_remove) => {
                let mut index_to_remove: i32 = -1;
                for (i, todo) in self.todos.iter().enumerate() {
                    if todo.title == title_to_remove {
                        index_to_remove = i as i32;
                        break;
                    }
                }

                if index_to_remove == -1 {
                    println!("No matching title was found!");
                } else {
                    self.todos.remove(index_to_remove as usize);
                    self.save();
                }
            }
            Err(_) => (),
        }
    }

    pub fn update(&mut self) {
        let options = self.get_titles();
        let title_to_update = Select::new("Enter the title to remove: ", options).prompt();

        match title_to_update {
            Ok(title_to_update) => {
                for todo in self.todos.iter_mut() {
                    if todo.title == title_to_update {
                        todo.edit();
                        break;
                    }
                }
                self.save();
            }
            Err(_) => (),
        }
    }

    pub fn filter_by_topic(&self) {
        let options = Self::get_topics();
        let topic_to_filter_by = Select::new("Enter the topic to filter by: ", options).prompt();

        match topic_to_filter_by {
            Ok(topic_to_filter_by) => {
                for todo in &self.todos {
                    if todo.topic == topic_to_filter_by {
                        println!("{}", todo);
                    }
                }
            }
            Err(_) => (),
        }
    }

    pub fn sort(&mut self) {
        self.todos.sort();
    }

    pub fn save(&mut self) {
        self.sort();

        let mut todo_vector = TodoVector { todo: Vec::new() };

        for todo in &self.todos {
            todo_vector.todo.push(todo.to_json_todo());
        }

        let json_string = serde_json::to_string_pretty(&todo_vector).unwrap();

        fs::write("todo.json", json_string).unwrap();
    }

    pub fn get_titles(&self) -> Vec<String> {
        let mut titles = Vec::new();

        for todo in &self.todos {
            titles.push(todo.title.to_string());
        }

        titles
    }

    pub fn get_topics() -> Vec<String> {
        let mut topics = Vec::new();
        let todos = Todos::parse();

        for todo in todos {
            topics.push(todo.topic.to_string());
        }

        // Removes duplicates
        topics.sort();
        topics.dedup();

        topics
    }

    pub fn late(&self) {
        for todo in &self.todos {
            if todo.is_late() {
                println!("{}", todo);
            }
        }
    }

    pub fn add_combo_box(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Filter by")
            .selected_text(format!("{}", self.selected_topic))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.selected_topic,
                    Topic::All,
                    format!("{}", Topic::All),
                );
                ui.selectable_value(
                    &mut self.selected_topic,
                    Topic::Late,
                    format!("{}", Topic::Late),
                );
                for topic in Todos::get_topics() {
                    ui.selectable_value(
                        &mut self.selected_topic,
                        Topic::Topic(topic.clone()),
                        topic,
                    );
                }
            });
        // if ui.button(RichText::new("+").size(20.).strong()).clicked() {
        //     egui::Window::new("My Window").show(ctx, |ui| {
        //         ui.label("Hello World!");
        //     });
        // }
    }

    pub fn display_todos(&mut self, ui: &mut egui::Ui) {
        for todo in &self.todos {
            match &self.selected_topic {
                Topic::All => todo.display(ui),
                Topic::Topic(topic) => {
                    if todo.topic.as_str() == topic {
                        todo.display(ui);
                    }
                }
                Topic::Late => {
                    if todo.is_late() {
                        todo.display(ui);
                    }
                }
            }
        }
    }
}

impl fmt::Display for Todos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for todo in &self.todos {
            write!(f, "{}", todo)?
        }
        Ok(())
    }
}

impl eframe::App for Todos {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Updates the list
        self.todos = Todos::parse();
        // Creates the ui
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("Future Tasks").size(25.).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    self.add_combo_box(ui);
                })
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| self.display_todos(ui));
            ctx.request_repaint_after(Duration::from_secs(60));
        });
    }
}

impl Default for Todos {
    fn default() -> Self {
        Todos {
            todos: Todos::parse(),
            selected_topic: Topic::All,
        }
    }
}
