use std::collections::HashMap;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use crate::{app::{App, Id}, todo::Todo};
use super::cli_utils;

#[derive(Parser)]
pub struct Args {
    /// shows only completed todos
    #[arg(long, short = 'c')]
    show_complete: bool,
    // #[arg(long, short = 'd')]
    // show_description: bool,
    /// shows only incompleted todos
    #[arg(long, short = 'i')]
    show_incomplete: bool,
    #[arg(long, short = 'f', default_value = "table")]
    format: OutputFormat,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
}

pub fn run(app: App, args: Args) -> Result<()> {
    #[allow(unused_variables)]
    let Args {
        show_complete,
        // show_description,
        show_incomplete,
        format
    } = args;

    let todos: HashMap<Id, Todo> = if show_complete {
        app.todos.into_iter().filter(|(_, t)| t.is_completed).collect()
    } else if show_incomplete {
        app.todos.into_iter().filter(|(_, t)| !t.is_completed).collect()
    } else {
        app.todos
    };

    match format {
        OutputFormat::Json => cli_utils::print_todos_json(todos),
        OutputFormat::Table => cli_utils::print_todos_table(todos),
    }

    Ok(())
}
