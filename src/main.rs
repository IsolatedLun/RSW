use std::{io::stdin, collections::HashMap};
use commands::help_command::HelpCommand;

const OPTION_IDENTIFIER: &str = "--";

mod cli;
mod commands {
    pub mod help_command;
};


fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();
    let mut input_parser = cli::InputParser::new(&buf);

    match input_parser.command.as_str() {
        "help" => HelpCommand::new()
    }
}
