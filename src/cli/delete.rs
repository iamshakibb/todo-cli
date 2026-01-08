use crate::app::{App};
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    id: usize,
}

pub fn run(mut app:App, args: Args) -> Result<()> {
    let Args { id } = args;
    let todo = app.get_todo(id).clone();
    if todo.is_none() {
        println!("Todo with id {} not found", id);
    } else {
        app.delete_todo(id);
    }
    Ok(())
}
