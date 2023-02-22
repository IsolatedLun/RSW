use std::{collections::HashMap, fs::File, io::Write, io::Read, vec, process::Command};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

struct Config {
    properties: Option<ConfigProperties>
}

impl Config {
    pub fn new() -> Self {
        Config { properties: None }
    }

    pub fn load_config(&mut self) {
        let mut json_config_file: Option<File> = match File::open("config.json") {
            Ok(f) => Some(f),
            Err(_) => {
                println!("[INFO] Creating new config file");
                
                match File::create("config.json") {
                    Ok(f) => Some(f),
                    Err(_) => {
                        println!("[ERROR] Failed creating config file");
                        None
                    }
                }
            }
        };

        if json_config_file.is_none() {
            return;
        }

        let mut text_data: String = String::new();

        json_config_file.unwrap().read_to_string(&mut text_data).unwrap();

        // serde_json::to_string(&ConfigProperties::new());
        // WIP
        // if config is invalid, prompt the user to reset or manually fix it themselves 
        let json_data: ConfigProperties = match serde_json::from_str(&text_data) {
            Ok(data) => data,
            Err(_) => panic!("Invalid json format for config")
        };

        self.properties = Some(json_data);
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigProperties {
    aliases: Vec<(usize, String)>
}

impl ConfigProperties {
    pub fn new() -> Self {
        ConfigProperties { aliases: vec![] }
    }
}

pub struct Manager {
    workshop: HashMap<usize, (String, Vec<usize>)>,
    config: Option<Config>
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            workshop: HashMap::new(),
            config: Some(Config::new())
        }
    }

    pub fn add_items(&mut self, app_id: usize, items: Vec<usize>) {
        if items.len() == 0 {
            return;
        }
        
        let items_len = items.len();

        if self.workshop.contains_key(&app_id) {
            self.workshop.get_mut(&app_id).unwrap().1.extend(items);
        }

        else {
            self.workshop.insert(app_id, (String::from("lol"), items));
        }

        println!("Added {} items for '{}'", items_len, self.alias(&self.workshop.get(&app_id).unwrap().0));
    }

    pub fn export(&mut self) -> Vec<(usize, String)> {
        let mut contents_list: Vec<(usize, String)> = vec![];

        for (app_id, (app_name, item_ids)) in self.workshop.iter() {
            let rand_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
            let mut contents = String::from("steamcmd +login anonymous +workshop_download_item");

            for id in item_ids.iter() {
                contents.push_str(format!(" {:?}", id).as_str());
            }

            let mut file = File::create(format!(
                "{}-{}-{}.txt", app_id, app_name, rand_string
            )).unwrap();

            file.write_all(contents.as_bytes()).unwrap();
            contents_list.push((*app_id, contents));
        }

        contents_list
    }

    pub fn download(&mut self) {
        for (app_id, command) in self.export() {
            match Command::new(command).spawn() {
                Ok(_res) => println!("[SUCCESS] Downloaded items for '{}'", app_id),
                Err(_err) => println!("[ERROR] Couldn't downloads items for '{}'", app_id)
            }
        }
    }

    pub fn alias(&self, alias_or_app_id: &String) -> String {
        if alias_or_app_id.chars().all(char::is_alphabetic) {
            alias_or_app_id.to_owned()
        }

        else {
            String::from("LMAO")
        }
    }
}