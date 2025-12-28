use todo_cli::app::App;
use run_cli::run_cli;

mod run_cli {
    mod add {}
    mod delete {}
    mod complete {}
    mod cli_utils {
        use std::collections::HashMap;

        use todo_cli::{app::Id, todo::Todo};

        pub fn print_todos (
            todos: HashMap<Id, Todo>,
            // show_description: bool,
        ) {
            println!("{}", serde_json::to_string(&todos).expect("Failed to serialize tasks to JSON"))
        }
    }
    use clap::Parser;
    mod ls {
        use std::collections::HashMap;
        use anyhow::Result;
        use clap::Parser;
        use todo_cli::{app::{App, Id}, todo::Todo};
        use super::cli_utils;

        #[derive(Parser)]
        pub struct Args {
            #[arg(long, short = 'c')]
            show_complete: bool,
            #[arg(long, short = 'd')]
            show_description: bool,
             #[arg(long, short = 'i')]
             show_incomplete: bool
        }



        pub fn run(app: App, args: Args) -> Result<()> {
            #[allow(unused_variables)]
            let Args {
                show_complete,
                show_description,
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
    }


    use todo_cli::app::App;
    use anyhow::Result;

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
    }

    pub fn run_cli(app:App) -> Result<()> {
        let args = Args::parse();
        match args.command {
            Command::Ls(args) => ls::run(app,args),
            // Command::Add(args) => add::run(args),
            // Command::Delete(args) => delete::run(args),
            // Command::Complete(args) => complete::run(args),
        }
    }
}




fn main () {
    let app = App::new();
    let res = if std::env::args().len() > 1 {
            run_cli(app)
    } else {
        run_cli(app)
    };

    if let Err(e) = res {
        eprintln!("{}", e);
    }
}
