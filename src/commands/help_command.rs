use crate::cli::InputParser;

pub struct HelpCommand<'a> {
    pub data: InputParser<'a >,
}

impl<'a > HelpCommand<'a> {
    pub fn new(data: InputParser) -> Self {
        HelpCommand { data }
    }
}
