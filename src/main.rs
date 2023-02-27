use std::io::{self, stdin, Write};
use std::process::exit;

use crate::utils::{log, LogLevel};
use crate::commands::Command;

use crate::commands::alias::AliasCommand;
use crate::commands::search::SearchCommand;
use crate::commands::help::HelpCommand;

const VERSION: f32 = 1.0;
const STEAMCMD_DIR: &str = r#"C:/Users/user/Desktop/steamcmd/steamcmd.exe"#;

mod cli;
mod manager;
mod utils;
mod commands;


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
