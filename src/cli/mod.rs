use clap::Parser;
use crate::app::App;
use anyhow::Result;

mod ls;
mod add;
mod delete;
mod complete;
mod edit;
mod cli_utils;


#[derive(Parser)]
#[command(name = "todo-cli", author, version, about = "A simple command-line todo manager",
    long_about = "A simple command-line todo manager for adding, listing, editing, completing, and deleting tasks.")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Lists all the tasks
    Ls(ls::Args),
    /// Add a new task
    Add(add::Args),
    /// Delete a task by ID
    Delete(delete::Args),
    /// Mark a task as completed
    Complete(complete::Args),
    /// Edit an existing task
    Edit(edit::Args),
}

pub fn run_cli(app:App) -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Ls(args) => ls::run(app,args),
        Command::Add(args) => add::run(app,args),
        Command::Delete(args) => delete::run(app,args),
        Command::Complete(args) => complete::run(app,args),
        Command::Edit(args) => edit::run(app,args),
    }
}
