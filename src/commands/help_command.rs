use crate::cli::InputParser;

pub struct HelpCommand<'a> {
    pub data: &'a InputParser<'a >,
}

impl<'a > HelpCommand<'a> {
    pub fn new(data: &'a InputParser) -> Self {
        HelpCommand { data }
    }

    pub fn run(&self) {
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
            "aliases", "(show, add [number])", "-", "-"
        );

        println!(
            "{0: <30} | {1: <30} | {2: <30} | {3: <30}",
            "search", "(id, alias) [query]", "(pages [number])", "Search workshop items"
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
}
