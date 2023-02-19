use std::collections::HashMap;

pub struct InputParser<'a> {
    pub command: String,
    pub args: Vec<&'a str>,
    pub options: HashMap<String, String>
}


impl<'a> InputParser<'a> {
    pub fn new(input: &'a String) -> Self {
        let list = input.split(' ').into_iter().collect::<Vec<_>>();
        assert!(list.len() > 0);
        assert!(!list[0].starts_with("--"), "First argument must be a command");

        let mut option_offset: usize = 1;
        for item in &list[1..] {
            if item.starts_with("--") {
                break
            }

            option_offset += 1
        }

        let mut options_hashmap: HashMap<String, String> = HashMap::new();
        for item in list[option_offset..].chunks(2) {
            assert!(item.len() == 2);
            assert!(!item[1].starts_with("--"), "Argument for '{}' cannot be an option", item[0]);

            options_hashmap.insert(item[0].to_string(), item[1].to_string());
        };

        InputParser { 
            command: list[0].to_owned(), 
            args: list[1..option_offset].to_owned(), 
            options: options_hashmap
        }
    }
}