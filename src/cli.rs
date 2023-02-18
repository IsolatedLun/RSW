use std::collections::HashMap;

use crate::OPTION_IDENTIFIER;

pub struct InputParser<'a> {
    pub command: String,
    pub args: Vec<&'a str>,
    pub options: HashMap<&'a str, &'a str>
}


impl<'a> InputParser<'a> {
    pub fn new(input: &'a String) -> Self {
        let list = input.split(' ').into_iter().collect::<Vec<_>>();
        assert!(list.len() > 0);
        assert!(!list[0].starts_with(OPTION_IDENTIFIER), "First argument must be a command");

        let mut option_offset: usize = 1;
        for item in &list[1..] {
            if item.starts_with(OPTION_IDENTIFIER) {
                break
            }

            option_offset += 1
        }

        let mut options_hashmap: HashMap<&'a str, &'a str> = HashMap::new();
        for item in list[option_offset..].chunks(2) {
            assert!(item.len() == 2);
            assert!(!item[1].starts_with(OPTION_IDENTIFIER), "Argument for '{}' cannot be an option", item[0]);

            options_hashmap.insert(item[0], item[1]);
        };

        InputParser { 
            command: list[0].to_owned(), 
            args: list[1..option_offset].to_owned(), 
            options: options_hashmap.to_owned()
        }
    }
}