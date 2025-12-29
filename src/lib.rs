pub mod app;
pub mod todo;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

use crate::app::Id;
use crate::todo::Todo;

pub const DIR_PATH:&'static str = "todo_list";
pub const FILE_PATH:&'static str = "todo_list/todos.json";

pub fn file_checker(){
    if !Path::new(DIR_PATH).exists() {
        fs::create_dir_all(DIR_PATH)
            .expect("unable to create todo_list directory");
    }

    if !Path::new(FILE_PATH).exists() {
        fs::write(FILE_PATH, "{}")
            .expect("unable to create todos.json file");
    }
}

pub fn get_todos_from_file() -> HashMap<Id,Todo> {
    let todo_data = fs::read_to_string(FILE_PATH).expect("file to read todo.json");
    let todo_list: HashMap<Id, Todo> = serde_json::from_str(&todo_data)
        .unwrap_or_else(|_| HashMap::new());

    todo_list
}
