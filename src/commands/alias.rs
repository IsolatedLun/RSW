use crate::{cli::InputParser, utils::{log, LogLevel}, manager::Config, commands::Command};

pub struct AliasCommand<'a> {
    pub data: InputParser,
    pub config: &'a mut Config
}

impl<'a> Command<'a, ()> for AliasCommand<'a> {
    fn new(config: &'a mut Config, data:  InputParser) -> Self {
        AliasCommand { data, config }
    }

    fn run(&mut self) {
        let assertion = self.assert();
        if assertion.is_err() {
            log(
                LogLevel::ERR,
                format!("{}", assertion.unwrap_err())
            );
            return;
        }
        match self.data.args[0].as_str() {
            "show" => self.display_aliases(),
            "set" => self.set(),
            "remove" => self.remove(),
            _ => log(
                LogLevel::ERR, 
                format!("'{}' is not a valid command", self.data.args[0])
            ),
        }
    }

    fn assert(&self) -> Result<(), String> {
        if self.data.args.len() == 0 {
            return Err(String::from("Insufficient arguments"))
        }

        Ok(())
    }
}

impl<'a> AliasCommand<'a> {
    pub fn display_aliases(&self) {
        match self.config.get_props_ref() {
            Some(props) => {
                println!("{}", "-".repeat(60));
                
                for (k, v) in props.aliases.to_owned().into_iter() {
                    println!(
                        "{0: <30} | {1: <30}",
                        k, v
                    );
                    println!("{}", "-".repeat(60));
                }
            },

            None => log(
                LogLevel::WARN, 
                format!("No aliases found")
            )
        }
    }

    pub fn set(&mut self) {
        if self.data.args.len() < 3 {
            log(
                LogLevel::ERR, 
                format!("Insufficient arguments")
            );
            return;
        }

        if !self.data.args[2].chars().all(char::is_numeric) {
            log(
                LogLevel::ERR, 
                format!("App id must only contains numbers")
            );
            return;
        }

        match self.config.get_props_mut() {
            Some(props) => props.set_alias_by_values(
                self.data.args[1].clone(), self.data.args[2].clone()
            ),
            None => log(
                LogLevel::WARN, 
                format!("No config found")
            )
        }
    }

    pub fn remove(&mut self) {
        if self.data.args.len() < 2 {
            log(
                LogLevel::ERR, 
                format!("Insufficient arguments")
            );
            return;
        }

        match self.config.get_props_mut() {
            Some(props) => props.remove_alias(self.data.args[1].clone()),
            None => log(
                LogLevel::WARN, 
                format!("No config found")
            )
        }
    }
}