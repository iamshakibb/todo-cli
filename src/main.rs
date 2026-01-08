use todo_cli::app::App;
use todo_cli::cli;

fn main () {
    let app = App::new();
    let res = if std::env::args().len() > 1 {
            cli::run_cli(app)
    } else {
        cli::run_cli(app)
    };

    if let Err(e) = res {
        eprintln!("{}", e);
    }
}
