use std::{io::{self, Write}};

use reqwest::{blocking::Response};
use scraper::{ElementRef};

use crate::{cli::InputParser, manager::Config, commands::command::Command, utils::{log, LogLevel}};

pub struct SearchCommand<'a> {
    pub data: InputParser,
    pub config: &'a mut Config,
    app_id: String
}

impl<'a> Command<'a, (String, Vec<usize>)> for SearchCommand<'a> {
    fn new(config: &'a mut Config, data: InputParser) -> Self {
        SearchCommand { data, config, app_id: String::new() }
    }

    fn run(&mut self) -> (String, Vec<usize>) {
        let assertion = self.assert();
        if assertion.is_err() {
            log(
                LogLevel::ERR,
                format!("{}", assertion.unwrap_err())
            );
            return (String::new(), vec![]);
        }

        let app_id_res = self.try_get_app_id();
        if app_id_res.is_none() {
            log(
                LogLevel::ERR, 
                format!("App id not found for '{}'", self.data.args[0])
            );
            return (String::new(), vec![]);
        }
        self.app_id = app_id_res.unwrap();

        let url = self.create_url(1);

        let req: Option<Response> = match reqwest::blocking::get(&url) {
            Ok(res) => Some(res),
            Err(_err) => None
        };

        if req.is_none() {
            log(
                LogLevel::ERR, 
                format!("Couldn't access steam workshop at this time")
            );
            return (String::new(), vec![])
        }

        let html = scraper::Html::parse_document(&req.unwrap().text().unwrap());
        let app_header_selector = scraper::Selector::parse(".apphub_HomeHeaderContent").unwrap();

        if html.select(&app_header_selector).count() == 0 {
            log(
                LogLevel::ERR, 
                format!("An app with an appID of '{}' does not exist", self.app_id.trim())
            );
            return (String::new(), vec![]);
        }

        if self.config.properties.is_some() {
            let title_selector = scraper::Selector::parse("title").unwrap();
            let title_el: ElementRef = *(html.select(&title_selector).collect::<Vec<ElementRef>>())
                .get(0).unwrap();

            self.config.properties.as_mut().unwrap().set_alias(self.app_id.clone(), title_el);
        }
        

        let workshop_item_selector = scraper::Selector::parse(".workshopItem").unwrap();
        let mut workshop_items: Vec<ElementRef> = html.select(&workshop_item_selector).collect();

        self.display_workshop_items(&mut workshop_items);

        let mut buf = String::new();

        print!("Select by index (0 1 2) > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();

        let indices: Vec<usize> = buf.split_whitespace()
            .filter(|x| x.chars().all(char::is_numeric))
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut selected_ids: Vec<usize> = vec![];

        for idx in indices {
            match workshop_items.get(idx) {
                Some(item) => selected_ids.push(
                    self.get_item_id(item).parse::<usize>().unwrap()
                ),
                None => log(
                    LogLevel::ERR, 
                    format!("Item with an index of '{}' does not exist", idx)
                )
            }
        }

        println!("{:?}", selected_ids);

        return (self.app_id.trim().to_string(), selected_ids)

    }

    fn assert(&self) -> Result<(), String> {
        if self.data.args.len() == 0 {
            return Err(String::from("Insufficient arguments"))
        }

        Ok(())
    }
}

impl<'a> SearchCommand<'a> {
    fn get_item_id(&self, workshop_item: &'a ElementRef) -> &str {
        workshop_item.select(&scraper::Selector::parse(".ugc").unwrap())
            .next().unwrap()
            .value().attr("data-publishedfileid").unwrap()
    }

    fn try_get_app_id(&mut self) -> Option<String> {
        if self.data.args[0].chars().all(char::is_numeric) {
            return Some(self.data.args[0].to_owned());
        }

        match &self.config.properties {
            Some(props) => props.get_app_id_by_name(self.data.args[0].to_owned()),
            None => None
        }
    }

    fn display_workshop_items(&self, items: &mut Vec<ElementRef>) {
        for (i, workshop_item) in items.into_iter().enumerate() {
            let item_name_selector = scraper::Selector::parse(".workshopItemTitle").unwrap();
            let item_name: String = workshop_item.select(&item_name_selector).flat_map(|el| el.text()).collect();

            let item_id = self.get_item_id(workshop_item);

            println!("{}) {} [{}]", i, item_name, item_id);
        }
    }

    fn create_url(&self, page: usize) -> String {
        format!(
            "https://steamcommunity.com/workshop/browse/?appid={}&searchtext={}&days={}&p={}",
            self.app_id, 
            self.data.args.get(1).or(Some(&String::from(""))).unwrap(),
            self.data.options.get("--days").or(Some(&String::from("-1"))).unwrap(),
            self.data.options.get("--pages").or(Some(&page.to_string())).unwrap(),
        )
    }
}
