use crate::{manager::Config, cli::InputParser};

pub trait Command<'a, Run> {
    fn new(config: &'a mut Config, data: InputParser) -> Self;
    fn run(&mut self) -> Run;
    fn assert(&self) -> Result<(), String>; 
}