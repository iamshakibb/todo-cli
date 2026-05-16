use anyhow::Result;
use clap::Parser;
use crate::{app::{App}, todo};

#[derive(Parser)]
#[command(about = "Add a new todo task")]
pub struct Args {
    /// Title of the todo task
    title: String,
    /// Optional description for the task
    #[arg(long,short = 'd')]
    description: Option<String>
}

pub fn run(mut app:App, args: Args) -> Result<()>{
    let Args { title, description } = args;
    let id = app.todos.len() + 1;
    let todo = todo::Todo{ id, title, description: description, is_completed: false };
    app.add_todo(todo);
    Ok(())
}
