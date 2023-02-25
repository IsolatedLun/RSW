use std::io::{self, stdin, Write};
use std::process::exit;

use commands::command::Command;
use commands::help_command::HelpCommand;
use commands::search_command::SearchCommand;
use commands::alias_command::AliasCommand;

use crate::utils::{log, LogLevel};

const VERSION: f32 = 1.0;

mod cli;
mod manager;
mod utils;
mod commands {
    pub mod command;
    pub mod help_command;
    pub mod search_command;
    pub mod alias_command;
}


fn main() {
    let mut manager = manager::Manager::new();
    manager.config.load_config();

    loop {
        let mut buf = String::new();

        print!("RSWC (v{:?})> ", VERSION);
        
        io::stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();

        let input_parser = cli::InputParser::new(buf);

        match input_parser.command.as_str().trim() {
            "help" => HelpCommand::new(&mut manager.config,input_parser).run(),
            "search" => {
                let (app_id, items) = SearchCommand::new(&mut manager.config, input_parser).run();
                manager.add_items(app_id, items);
            },
            "export" => {
                manager.export();
            }
            "download" => {
                manager.download();
            },
            "aliases" => {
                AliasCommand::new(&mut manager.config, input_parser).run();
            },
            "exit" => {
                log(
                    LogLevel::EXIT, 
                    format!("Saving and exitting...")
                );
                manager.save();
                
                exit(0);
            }
            _ => log(
                LogLevel::ERR, 
                format!("'{}' is not a valid command", input_parser.command.trim())
            )
        }
    }
}
