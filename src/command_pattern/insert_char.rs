use std::sync::{Arc, Mutex};
use super::command;

pub struct InsertChar {
    text: Arc<Mutex<String>>,
    character: char
}

impl InsertChar {
    pub fn new (text: Arc<Mutex<String>>, character: char) -> InsertChar {
        Self {
            text,
            character
        }
    }
}
impl command::Command for InsertChar {

    fn execute(&mut self ) {
        self.text.lock().unwrap().push(self.character);
    }

    fn rollback(&mut self) {
        let len = self.text.lock().unwrap().len();
        self.text.lock().unwrap().pop();
    }
}