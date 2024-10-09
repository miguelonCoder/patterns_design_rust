use crate::factory::base_source::BaseSource;
use crate::factory::document::Document;
use crate::factory::txt_document::TxtDocument;

pub struct TxtSource {
    source: String
}

impl TxtSource {
    pub fn new(source: String) -> TxtSource {
        Self {
            source
        }
    }
}

impl BaseSource for TxtSource {
    fn create_document(&self) -> Box<dyn Document> {
        Box::new(TxtDocument::new())
    }

    fn export_document(&self) -> String {
        String::from("Hola desde el documento TXT.")
    }
}