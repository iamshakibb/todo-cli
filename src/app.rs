use std::collections::HashMap;
use std::fs;

use crate::{FILE_PATH, file_checker, get_todos_from_file, todo::Todo};

pub type Id = usize;

pub struct App {
    pub todos: HashMap<Id, Todo>,
}

impl App {
    pub fn new() -> App {
        file_checker();
        let todos = get_todos_from_file();
        App { todos }
    }

    pub fn get_todo(&self, id: Id) -> Option<&Todo> {
        self.todos.get(&id)
    }

    pub fn add_todo(&mut self, todo: Todo) -> Id {
        let id = self.todos.len() + 1;
        self.todos.insert(id, todo);
        self.persist();
        id
    }

    pub fn delete_todo(&mut self, id: Id) {
        if self.todos.remove(&id).is_some() {
            self.persist();
        } else {
            println!("Todo with id {} not found", id);
        }
    }

    pub fn set_completed(&mut self, id: Id, completed: bool) -> Option<&Todo> {
        let todo = match self.todos.get_mut(&id) {
            Some(todo) => todo,
            None => {
                println!("Todo with id {} not found", id);
                return None;
            }
        };

        if completed {
            todo.set_completed();
        } else {
            todo.set_uncompleted();
        }

        self.persist();
        // return immutable todo
        self.get_todo(id)
    }

    pub fn toggle_completed(&mut self, id: Id) -> Option<&Todo> {
        let todo = self.todos.get_mut(&id);
        if todo.is_none() {
            println!("Todo with id {} not found", id);
            return None;
        }
        todo.unwrap().toggle_completed();
        self.persist();
        // return immutable todo
        self.get_todo(id)
    }

    fn persist(&self) {
        let updated_file =
            serde_json::to_string_pretty(&self.todos).expect("Failed to serialize todos");

        fs::write(FILE_PATH, updated_file).expect("Failed to write todos.json");
    }
}
