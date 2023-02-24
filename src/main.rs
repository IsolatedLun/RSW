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
    let mut manager = manager::Manager::new();
    manager.config.load_config();

    loop {
        let mut buf = String::new();

        print!("RSWC (v{:?})> ", VERSION);
        
        io::stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();

        let input_parser = cli::InputParser::new(&buf);

        // TODO:
        // Add manager that handles file system, aliases, and downloading...
        match input_parser.command.as_str().trim() {
            "help" => HelpCommand::new(&input_parser).run(),
            "search" => {
                let (app_id, items) = SearchCommand::new(&mut manager.config, &input_parser).run();
                manager.add_items(app_id, items);
            },
            "export" => {
                manager.export();
            }
            "download" => {
                manager.download();
            },
            "aliases" => {
                match input_parser.args[0].trim() {
                    "show" => {
                        if manager.config.properties.is_some() {
                            println!("> {:?}", manager.config.properties.as_ref().unwrap().aliases)
                        }
                    },
                    _ => ()
                }
            },
            "exit" => {
                println!("[EXIT] Saving and exitting...");
                manager.save();
                
                exit(0);
            }
            _ => println!("'{}' is not a valid command", input_parser.command.trim())
        }
    }
}
