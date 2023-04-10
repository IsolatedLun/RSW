use crate::cli::InputParser;
use crate::commands::Command;
use crate::manager::Config;

pub struct HelpCommand<'a> {
    pub data: InputParser,
    pub config: &'a mut Config
}

impl<'a> Command<'a, ()> for HelpCommand<'a> {
    fn new(config: &'a mut Config, data: InputParser) -> Self {
        HelpCommand { data, config }
    }

    // WIP:
    // Store help commands in a hashmap and iterate through it instead of manually coding it in
    fn run(&mut self) {
        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "Command", "Args", "Options", "Description"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "help", "-", "-", "-"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "aliases", "show, add [name] [appID], remove [name]", "-", "-"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "search", "[appAlias] [query]", "--pages [number]", "Search workshop items (sorted by top)"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "export", "-", "-", "Exports all added items to a valid steamcmd command in txt"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "convert", "[appAlias] [filePath]", "--file", "Parses a file and converts it to the valid format"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "download", "-", "--file", "Downloads items via steamcmd (exports automatically)"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "exit", "-", "-", "Saves current data and exits"
        );
    }

    fn assert(&self) -> Result<(), String> {
        Ok(())
    }
}
