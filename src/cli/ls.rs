use std::collections::HashMap;
use anyhow::Result;
use clap::Parser;
use crate::{app::{App, Id}, todo::Todo};
use super::cli_utils;

#[derive(Parser)]
pub struct Args {
    #[arg(long, short = 'c')]
    show_complete: bool,
    // #[arg(long, short = 'd')]
    // show_description: bool,
    #[arg(long, short = 'i')]
    show_incomplete: bool
}

pub fn run(app: App, args: Args) -> Result<()> {
    #[allow(unused_variables)]
    let Args {
        show_complete,
        // show_description,
        show_incomplete
    } = args;

    let todos: HashMap<Id, Todo> = if show_complete {
        app.todos.into_iter().filter(|(_, t)| t.is_completed).collect()
    } else if show_incomplete {
        app.todos.into_iter().filter(|(_, t)| !t.is_completed).collect()
    } else {
        app.todos
    };

    cli_utils::print_todos(todos);
    Ok(())
}
