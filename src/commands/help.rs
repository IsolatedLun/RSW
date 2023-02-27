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
            "aliases", "(show/add [number])", "-", "-"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "search", "(appID/appName) [query]", "--pages [number]", "Search workshop items"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "export", "-", "-", "Exports the download command to a txt file"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "download", "-", "-", "Downloads items via steamcmd (exports automatically)"
        );
    }

    fn assert(&self) -> Result<(), String> {
        Ok(())
    }
}
