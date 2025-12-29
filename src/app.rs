use std::collections::HashMap;
use crate::{FILE_PATH, file_checker, get_todos_from_file, todo::Todo};
use std::fs;

pub type Id = usize;

pub struct App {
    pub todos: HashMap<Id, Todo>,
}

impl App {
    pub fn new() -> App {
        file_checker();
        let todos = get_todos_from_file();
        App {
            todos,
        }
    }

    pub fn get_todo(&self, id: Id) -> Option<&Todo> {
        self.todos.get(&id)
    }

    pub fn add_todo(&mut self, todo: Todo) -> Id {
        file_checker();
        let mut todos = get_todos_from_file();
        let id = self.todos.len() + 1;
        todos.insert(id, todo);
        let update_file = serde_json::to_string_pretty(&todos).expect("Failed to serialize");
        fs::write(FILE_PATH, update_file).expect("Failed to write todos.json");
        // self.todos.insert(id, todo);
        id
    }

    pub fn delete_todo(&mut self, id: Id) {
        self.todos.remove(&id);
    }

    pub fn update_todo(&mut self, id: Id, todo: Todo) {
        self.todos.insert(id, todo);
    }

    // pub fn set_completed(&mut self, id: Id, completed: bool) -> Option<&Todo> {
    //     let todo = if completed {
    //         self.todos.get_mut(&id)?.set_completed()
    //     } else {
    //         self.todos.get_mut(&id)?.set_uncompleted()
    //     };

    //     Some(todo)
    // }

    pub fn toggle_completed(&mut self, id: Id) -> Option<&Todo> {
        let todo = self.todos.get_mut(&id)?.toggle_completed();
        Some(todo)
    }
}
