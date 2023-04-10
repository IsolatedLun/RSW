use std::{fs::File, io::Read};

use crate::{cli::InputParser, utils::{log, LogLevel}, manager::Config, commands::Command};

pub struct ConvertCommand<'a> {
    pub data: InputParser,
    pub config: &'a mut Config
}

impl<'a> Command<'a, (String, Vec<usize>)> for ConvertCommand<'a> {
    fn new(config: &'a mut Config, data:  InputParser) -> Self {
        ConvertCommand { data, config }
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
        
        let file = File::open(self.data.args[1].clone());
        if file.is_err() {
            log(
                LogLevel::ERR,
                format!("File with path {} does not exist", self.data.args[1])
            );
            return (String::new(), vec![]);
        }

        let mut buf = String::new();
        file.unwrap().read_to_string(&mut buf);

        let parsed: Vec<usize> = match self.data.args[1].split(".").last().unwrap() {
            "txt" => buf.split(",").map(|x| x.to_string().parse::<usize>().unwrap())
                                .collect::<Vec<usize>>(),
            _ => {
                log(
                    LogLevel::ERR,
                    format!("File with type {} not supported", self.data.args[1])
                );

                vec![]
            }
        };

        return (self.data.args[0].clone(), parsed);
    }

    fn assert(&self) -> Result<(), String> {
        if self.data.args.len() == 0 {
            return Err(String::from("Insufficient arguments"))
        }

        Ok(())
    }
}

impl<'a> ConvertCommand<'a> {
 
}