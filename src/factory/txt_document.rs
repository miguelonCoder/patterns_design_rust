use std::cell::RefCell;
use std::rc::Rc;
use chrono::{DateTime, Utc};
use crate::factory::document::Document;

pub struct TxtDocument {
    date_created: DateTime<Utc>,
    content: Rc<RefCell<String>>,
    source: String
}
impl TxtDocument {
    pub fn new() -> TxtDocument {
        Self {
            date_created: Utc::now(),
            content: Rc::new(RefCell::new(String::from("Contenido inicial de mi documento txt"))),
            source: String::from("Source TXT")
        }
    }
}
impl Document for TxtDocument {
    fn read_content(&self) -> Rc<RefCell<String>> {
        self.content.clone()
    }

    fn write_content(&self, content: String) {
        self.content.borrow_mut().push_str(&content);
    }
}