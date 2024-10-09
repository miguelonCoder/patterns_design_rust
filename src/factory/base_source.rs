use crate::factory::document::Document;

pub trait BaseSource {
    fn create_document(&self) -> Box<dyn Document>;

    fn export_document(&self) -> String;

    fn initialize(&self) -> Box<dyn Document> {
        self.create_document()
    }
}