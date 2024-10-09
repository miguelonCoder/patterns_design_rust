use std::cell::RefCell;
use std::rc::Rc;

pub trait Document {
    fn read_content(&self) -> Rc<RefCell<String>>;

    fn write_content(&self, content: String);
}