use std::{collections::HashMap, fs::File, io::Write, io::Read, vec, process::Command, path::Path};
use rand::distributions::{Alphanumeric, DistString};
use scraper::ElementRef;
use serde::{Deserialize, Serialize};

pub struct Manager {
    workshop: HashMap<usize, (String, Vec<usize>)>,
    pub config: Config
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            workshop: HashMap::new(),
            config: Config::new()
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

    pub fn save(&self) {
        match &self.config.properties {
            Some(data) => {
                let str_data = serde_json::to_string(&data).unwrap();
                
                std::fs::write("config.json", str_data).unwrap();
            },
            None => unreachable!()
        }
    }
}

// ==========================
// Config
// ==========================
pub struct Config {
    pub properties: Option<ConfigProperties>
}

impl Config {
    pub fn new() -> Self {
        Config { properties: None }
    }

    pub fn load_config(&mut self) {
        let res = self.create_config_file();

        if res.is_err() {
            return;
        }

        let mut json_config_file = File::open("config.json").unwrap();
        let mut text_data: String = String::new();

        json_config_file.read_to_string(&mut text_data).unwrap();

        // WIP
        // if config is invalid, prompt the user to reset or manually fix it themselves 
        let json_data: ConfigProperties = match serde_json::from_str(&text_data) {
            Ok(data) => data,
            Err(_) => panic!("Invalid json format for config")
        };

        self.properties = Some(json_data);
    }

    fn create_config_file(&self) -> Result<(), ()> {
        match Path::new("config.json").exists() {
            true => Ok(()),
            false => {
                println!("[INFO] Creating config file");

                if File::create("config.json").is_err() {
                    return Err(())
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigProperties {
    #[serde(default)]
    pub aliases: HashMap<String, String>
}

impl ConfigProperties {
    #[warn(dead_code)]
    pub fn new() -> Self {
        ConfigProperties { aliases: HashMap::new() }
    }

    pub fn get_alias(&self, alias: String) -> Option<&String> {
        self.aliases.get(&alias)
    }

    pub fn set_alias(&mut self, app_id: String, title_el: ElementRef) {
        let name: String = title_el.text().map(|x| x).collect();

        self.aliases.insert(name.clone(), app_id.trim().to_string());
        println!("[INFO] Added alias for '{}'", name);
    }
}