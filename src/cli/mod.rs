use clap::Parser;
use crate::app::App;
use anyhow::Result;

mod ls;
mod add;
mod delete;
mod complete;
mod cli_utils {
    use std::collections::HashMap;
    use crate::{app::Id, todo::Todo};

    pub fn print_todos (
        todos: HashMap<Id, Todo>,
        // show_description: bool,
    ) {
        println!("{}", serde_json::to_string(&todos).expect("Failed to serialize tasks to JSON"))
    }
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Lists all the tasks
    Ls(ls::Args),
    Add(add::Args),
    Delete(delete::Args),
    Complete(complete::Args),
}

pub fn run_cli(app:App) -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Ls(args) => ls::run(app,args),
        Command::Add(args) => add::run(app,args),
        Command::Delete(args) => delete::run(app,args),
        Command::Complete(args) => complete::run(app,args),
    }
}
