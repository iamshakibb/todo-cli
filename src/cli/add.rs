use anyhow::Result;
use clap::Parser;
use crate::{app::{App}, todo};
#[derive(Parser)]
pub struct Args {
    title: String,
    #[arg(long)]
    description: Option<String>
}

pub fn run(mut app:App, args: Args) -> Result<()>{
    let Args { title, description } = args;
    let id = app.todos.len() + 1;
    let todo = todo::Todo{ id, title, description: description, is_completed: false };
    app.add_todo(todo);
    Ok(())
}
