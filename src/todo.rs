use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
}

impl Todo {
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_completed(&mut self) -> &Self {
        self.is_completed = true;
        self
    }

    pub fn set_uncompleted(&mut self) -> &Self {
        self.is_completed = false;
        self
    }

    pub fn toggle_completed(&mut self) -> &Self {
        if self.is_completed {
            self.set_uncompleted();
        } else {
            self.set_completed();
        }
        self
    }
}
