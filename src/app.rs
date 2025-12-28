use std::collections::HashMap;
use crate::todo::Todo;

pub type Id = usize;

pub struct App {
    pub todos: HashMap<Id, Todo>,
}

impl App {
    pub fn new() -> App {
        let todos: HashMap<Id, Todo> = HashMap::new();
        App {
            todos,
        }
    }

    pub fn get_todo(&self, id: Id) -> Option<&Todo> {
        self.todos.get(&id)
    }

    pub fn add_todo(&mut self, todo: Todo) -> Id {
        let id = self.todos.len() + 1;
        self.todos.insert(id, todo);
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
