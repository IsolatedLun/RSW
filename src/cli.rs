use std::collections::HashMap;

pub struct InputParser{
    pub command: String,
    pub args: Vec<String>,
    pub options: HashMap<String, String>
}


impl InputParser {
    pub fn new(input: String) -> Self {
        let mut list = InputParser::parse(input);

        assert!(list.len() > 0);
        assert!(!list[0].starts_with("--"), "First argument must be a command");

        let mut option_offset: usize = 0;
        for (i, mut item) in list.clone().into_iter().enumerate() {
            item = item.trim().to_string();
            list[i] = item.clone();

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

    fn parse(text: String) -> Vec<String> {
        let mut list: Vec<String> = vec![];
        let mut flag: usize = 0;

        let mut temp = String::new();
        for ch in text.chars() {
            if ch == ' ' && flag == 0 {
                list.push(temp.clone());
                temp.clear();
            }

            else if ch == ' ' && flag > 0 {
                temp.push(' ');
            }

            else if ch == '"' {
                flag += 1;
                if flag == 2 {
                    list.push(temp.clone());
                    temp.clear();
                    flag = 0;
                }
            }

            else {
                temp.push(ch);
            }
        }
        
        if !temp.is_empty() {
            list.push(temp.trim().to_string().clone());
        }

        list
    }
}