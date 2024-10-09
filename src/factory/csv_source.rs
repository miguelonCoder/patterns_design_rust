use crate::factory::base_source::BaseSource;
use crate::factory::csv_document::CsvDocument;
use crate::factory::document::Document;

pub struct CsvSource {
    separator: char,
    decimal: char,
    source: String
}
impl CsvSource {
    pub fn new(separator: char, decimal: char, source: String) -> CsvSource {
        Self {
            separator,
            decimal,
            source
        }
    }
}

impl BaseSource for CsvSource {
    fn create_document(&self) -> Box<dyn Document> {
        Box::new(CsvDocument::new())
    }

    fn export_document(&self) -> String {
        String::from("Exportando documento CSV")
    }
}