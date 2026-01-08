use clap::{Parser,ValueEnum};
use crate::app::App;
use anyhow::Result;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    id: usize,
    #[arg(short, long)]
    complete: CompleteStatus,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CompleteStatus {
    Complete,
    Incomplete,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        id,
        complete,
    } = args;
    let complete_bool = match complete {
        CompleteStatus::Complete => true,
        CompleteStatus::Incomplete => false,
    };

    let todo_id = app.set_completed(id, complete_bool);
    match todo_id {
        Some(_) => {
            print!("Todo updated successfully!")
        }
        None => {
            println!("Todo with id {} not found", id);
        }
    }

    Ok(())
}
