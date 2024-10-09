use std::sync::{Arc, Mutex};
use crate::command_pattern::command::Command;

pub struct TextFile {
    pub text: Arc<Mutex<String>>,
    pub commands: Vec<Box<dyn Command>>
}
impl TextFile {
    pub fn new() -> TextFile {
        Self {
            text: Arc::new(Mutex::new(String::new())),
            commands: Vec::new()
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

}
