use std::io::{self, Write};

use reqwest::{blocking::Response};
use scraper::{ElementRef};

use crate::cli::InputParser;

pub struct SearchCommand<'a> {
    pub data: &'a InputParser<'a >,
    app_id: String
}

impl<'a > SearchCommand<'a> {
    pub fn new(data: &'a InputParser) -> Self {
        let app_id: String  = match data.args[0].chars().all(char::is_alphabetic) {
            true => String::from("lol"),
            false => data.args[0].to_string(),
        };

        SearchCommand { data, app_id }
    }

    pub fn run(&self) -> (usize, Vec<usize>) {
        let url = self.create_url(1);

        let req: Option<Response> = match reqwest::blocking::get(&url) {
            Ok(res) => Some(res),
            Err(_err) => None
        };

        if req.is_none() {
            println!("[Error] > Couldn't access steam workshop at this time");
            return (0, vec![])
        }

        let html = scraper::Html::parse_document(&req.unwrap().text().unwrap());
        let app_header_selector = scraper::Selector::parse(".apphub_HomeHeaderContent").unwrap();

        if html.select(&app_header_selector).count() == 0 {
            println!("[Error] > An app with an appID of '{}' does not exist", self.app_id);
            return (0, vec![]);
        }

        let workshop_item_selector = scraper::Selector::parse(".workshopItem").unwrap();
        let mut workshop_items: Vec<ElementRef> = html.select(&workshop_item_selector).collect();

        self.display_workshop_items(&mut workshop_items);

        let mut buf = String::new();

        print!("Select items > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();

        let indices: Vec<usize> = buf.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut selected_ids: Vec<usize> = vec![];

        for idx in indices {
            match workshop_items.get(idx) {
                Some(item) => selected_ids.push(
                    self.get_item_id(item).parse::<usize>().unwrap()
                ),
                None => println!("[ERROR] Item with an index of '{}' does not exist", idx)
            }
        }

        println!("{:?}", selected_ids);

        return (self.app_id.trim().parse::<usize>().unwrap(), selected_ids)

    }

    fn get_item_id(&self, workshop_item: &'a ElementRef) -> &str {
        workshop_item.select(&scraper::Selector::parse(".ugc").unwrap())
            .next().unwrap()
            .value().attr("data-publishedfileid").unwrap()
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
            self.data.args.get(0).or(Some(&"")).unwrap(),
            self.data.options.get("--date").or(Some(&String::from("-1"))).unwrap(),
            self.data.options.get("--p").or(Some(&page.to_string())).unwrap(),
        )
    }
}
