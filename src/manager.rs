use std::{collections::HashMap, fs::File, io::Write, io::Read, vec, process::Command, path::Path};
use rand::distributions::{Alphanumeric, DistString};
use scraper::ElementRef;
use serde::{Deserialize, Serialize};

use crate::{utils::{underscorize, log, LogLevel}, STEAMCMD_DIR, cli::InputParser};

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

    pub fn export(&mut self) -> Vec<String> {
        let mut contents: Vec<String> = vec![String::from("+login"), String::from("anonymous")];
        let mut app_names: Vec<String> = vec![];

        if self.workshop.len() == 0 {
            return vec![];
        }

        for (app_id, (app_name, item_ids)) in self.workshop.iter() {
            app_names.push(app_name.clone());

            for id in item_ids.iter() {
                contents.push(format!("+workshop_download_item"));
                contents.push(app_id.to_string());
                contents.push(id.to_string())
            }
        }

        let rand_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        let mut file = File::create(format!(
            "{}-{}.txt", app_names.join("+"), rand_string
        )).unwrap();

        contents.push(String::from("+quit"));
        file.write_all(contents.join(" ").as_bytes()).unwrap();


        contents
    }

    pub fn download(&mut self, input: InputParser) {
        let mut data: Vec<String> = vec![];

        if input.options.contains_key("--file") {
            let file = File::open(input.options.get("--file").unwrap());

            match file {
                Ok(mut f) => {
                    let mut buf = String::new();
                    f.read_to_string(&mut buf).unwrap();

                    data.extend(
                        buf.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
                    )
                },
                Err(_) => log(
                    LogLevel::ERR, 
                    format!("File with the path '{}' does not exist", 
                        input.options.get("--file").unwrap()
                    )
                )
            };
        }

        else {
            data.extend(self.export())
        }

        if data.len() == 0 {
            log(
                LogLevel::INFO,
                format!("No items to download")
            );
            return;
        }

        // we div. by 3, since for each item 2 more strings get pushed
        let data_len = (data.len() / 3) - 1;
        let mut command = Command::new(STEAMCMD_DIR);
        
        command.args(data);
        match command.output() {
            Ok(_res) => {
                log(
                    LogLevel::SUCCESS, 
                    format!("Downloaded {} items", data_len)
                );

                if _res.stderr.len() > 0 {
                    log(
                        LogLevel::INFO,
                        String::from_utf8(_res.stderr).unwrap()
                    )
                }
            },
            Err(_err) => {
                log(
                    LogLevel::ERR, 
                    format!("Couldn't download items")
                );
                log(
                    LogLevel::ERR, 
                    format!("Cause: {}", _err.kind())
                );
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

    pub fn get_props_mut(&mut self) -> Option<&mut ConfigProperties> {
        self.properties.as_mut()
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

    pub fn set_alias_by_values(&mut self, name: String, app_id: String) {
        let name: String = underscorize(name);

        match self.aliases.insert(name.clone(), app_id) {
            None => log(
                LogLevel::INFO, 
                format!("Added alias for '{}'", name)
            ),
            _ => log(
                LogLevel::INFO, 
                format!("Updated alias for '{}'", name)
            ),
        }
    }

    pub fn remove_alias(&mut self, name: String) {
        let name: String = underscorize(name);

        match self.aliases.remove(&name) {
            Some(_) => log(
                LogLevel::WARN, 
                format!("Removed alias for '{}'", name)
            ),
            None => log(
                LogLevel::ERR, 
                format!("No alias found for '{}'", name)
            )
        }
    }
}