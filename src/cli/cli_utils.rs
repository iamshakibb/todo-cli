use std::collections::HashMap;
use tabled::{Table, grid::records::ExactRecords, settings::Style};

use crate::{app::Id, todo::Todo};

fn build_todo_rows(todos:HashMap<Id, Todo>) -> Vec<Todo> {
    todos.into_iter().map(|(id, todo)| Todo {
        id: id,
        title: todo.title,
        description: todo.description,
        is_completed: todo.is_completed,
    }).collect()
}

pub fn print_todos_table(todos: HashMap<Id,Todo>){
    let rows = build_todo_rows(todos);

    let count = rows.count_rows();

    if count == 0 {
        print!("\n No Todos found.\n");
        return;
    }

    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{table}");
}

pub fn print_todos_json(todos: HashMap<Id,Todo>){
    let rows = build_todo_rows(todos);

    let count = rows.count_rows();

    if count == 0 {
        print!("\n No Todos found.\n");
        return;
    }

    let json = serde_json::to_string_pretty(&rows);

    match json {
        Ok(json) => print!("\n{json}\n"),
        Err(_) => print!("Unable to print in json"),
    }

}
