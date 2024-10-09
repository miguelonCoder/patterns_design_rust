use std::sync::{Arc, Mutex};
use super::command;

pub enum BracketsCase {
    Braces,
    Brackets,
    Parenthesis,
    Quotes,
    SingleQuotes
}
pub struct AddBrackets {
    text: Arc<Mutex<String>>,
    case: BracketsCase,
}

impl AddBrackets {
    pub fn new (text: Arc<Mutex<String>>, case: BracketsCase) -> AddBrackets {
        Self {
            text,
            case
        }
    }

    fn add_characters(&self, first: char, last: char) {
        let mut text = self.text.lock().unwrap();
        text.insert(0, first);
        text.push(last);
    }
}

impl command::Command for AddBrackets {
    fn execute(&mut self) {
        match self.case {
            BracketsCase::Braces => self.add_characters('{', '}'),
            BracketsCase::Brackets => self.add_characters('[', ']'),
            BracketsCase::Parenthesis => self.add_characters('(', ')'),
            BracketsCase::Quotes => self.add_characters('"', '"'),
            BracketsCase::SingleQuotes => self.add_characters('\'', '\''),
        }

    }

    fn rollback(&mut self) {
        let mut text = self.text.lock().unwrap();
        text.pop();
        text.drain(0..1);
    }
}