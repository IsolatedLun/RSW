use std::io::{self, stdin, Write};
use std::process::exit;

use commands::help_command::HelpCommand;
use commands::search_command::SearchCommand;

const VERSION: f32 = 1.0;

mod cli;
mod manager;
mod commands {
    pub mod help_command;
    pub mod search_command;
}


fn main() {
    let mut buf = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    stdin().read_line(&mut buf).unwrap();

    let mut manager = manager::Manager::new();
    let input_parser = cli::InputParser::new(&buf);

    // TODO:
    // Add manager that handles file system, aliases, and downloading...
    match input_parser.command.as_str().trim() {
        "help" => HelpCommand::new(&input_parser).run(),
        "search" => {
            let (app_id, items) = SearchCommand::new(&input_parser).run();
            manager.add_items(app_id, items);
            manager.download();
        },
        "exit" => {
            exit(1);
        }
        _ => todo!()
    }
}
