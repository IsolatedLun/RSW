use std::{collections::HashMap, fs::File, io::Write, io::Read, vec, process::Command, path::Path};
use rand::distributions::{Alphanumeric, DistString};
use scraper::ElementRef;
use serde::{Deserialize, Serialize};

use crate::{utils::{underscorize, log, LogLevel}, STEAMCMD_DIR};

pub struct Manager {
    workshop: HashMap<String, (String, Vec<usize>)>,
    pub config: Config
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            workshop: HashMap::new(),
            config: Config::new()
        }
    }

    pub fn add_items(&mut self, app_id: String, items: Vec<usize>) {
        if items.len() == 0 {
            return;
        }
        
        let items_len = items.len();
        let name = self.config.get_props_ref()
                .unwrap()
                .get_name_by_app_id(app_id.clone())
                .unwrap();

        if self.workshop.contains_key(&app_id) {
            self.workshop.get_mut(&app_id).unwrap().1.extend(items);
        }

        else {
            self.workshop.insert(app_id.clone(), (name.to_owned(), items));
        }

        log(
            LogLevel::SUCCESS, 
            format!("Added {} items for '{}'", items_len, name)
        )
    }

    pub fn export(&mut self) -> Vec<(String, Vec<String>)> {
        let mut contents_list: Vec<(String, Vec<String>)> = vec![];

        for (app_id, (app_name, item_ids)) in self.workshop.iter() {
            let rand_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
            let mut contents: Vec<String> = vec![String::from("+login"), String::from("anonymous")];

            for id in item_ids.iter() {
                contents.push(format!("+workshop_download_item"));
                contents.push(app_id.to_string());
                contents.push(id.to_string())
            }
            contents.push(String::from("+quit"));

            let mut file = File::create(format!(
                "{}-{}-{}.txt", app_id, app_name, rand_string
            )).unwrap();

            file.write_all(contents.join(" ").as_bytes()).unwrap();
            contents_list.push((app_id.clone(), contents));
        }

        contents_list
    }

    pub fn download(&mut self) {
        for (app_id, content) in self.export() {
            let name = match self.config.get_props_ref() {
                Some(props) => props.get_name_by_app_id(app_id).unwrap(),
                None => app_id
            };

            let mut command = Command::new("C:/Users/user/Desktop/steamcmd/steamcmd.exe");
            command.args(content);

            match command.output() {
                Ok(_res) => {
                    println!("{}", String::from_utf8(_res.stderr).unwrap());
                    log(
                        LogLevel::SUCCESS, 
                        format!("Downloaded items for '{}'", name)
                    );
                },
                Err(_err) => {
                    println!("{}", _err.to_string());
                    log(
                        LogLevel::ERR, 
                        format!("Couldn't downloads items for '{}'", name)
                    );
                }
                
            }
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

    pub fn get_props_ref(&self) -> Option<&ConfigProperties> {
        self.properties.as_ref()
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
            Err(_) => {
                log(
                    LogLevel::ERR, 
                    format!("Invalid json format for config")
                );
                panic!("");
            }
        };

        self.properties = Some(json_data);
    }

    fn create_config_file(&self) -> Result<(), ()> {
        match Path::new("config.json").exists() {
            true => Ok(()),
            false => {
                log(
                    LogLevel::INFO, 
                    format!("Creating log file")
                );

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

    pub fn get_app_id_by_name(&self, name: String) -> Option<String> {
        self.aliases.get(&name).cloned()
    }

    pub fn get_name_by_app_id(&self, app_id: String) -> Option<String> {
        for (name, _app_id) in self.aliases.to_owned().into_iter() {
            if app_id == _app_id {
                return Some(name);
            }
        }

        None
    }

    pub fn set_alias(&mut self, app_id: String, title_el: ElementRef) {
        let name: String = underscorize(title_el.text().map(|x| x).collect());

        match self.aliases.insert(name.clone(), app_id.trim().to_string()) {
            None => log(
                LogLevel::INFO, 
                format!("Added alias for '{}'", name)
            ),
            _ => ()
        }
    }
}