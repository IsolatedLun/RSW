use std::{collections::HashMap, fs::File, io::Write};
use rand::distributions::{Alphanumeric, DistString};

pub struct Manager {
    workshop: HashMap<usize, (String, Vec<usize>)>
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            workshop: HashMap::new()
        }
    }

    pub fn add_items(&mut self, app_id: usize, items: Vec<usize>) {
        let items_len = items.len();

        if self.workshop.contains_key(&app_id) {
            self.workshop.get_mut(&app_id).unwrap().1.extend(items);
        }

        else {
            self.workshop.insert(app_id, (String::from("lol"), items));
        }

        println!("Added {} items for '{}'", items_len, self.alias(&self.workshop.get(&app_id).unwrap().0));
    }

    pub fn download(&mut self) {
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