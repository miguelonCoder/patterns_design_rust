
pub trait NavView {
    fn get_description(&self) -> String;

}

pub struct CompactNavView;

impl NavView for CompactNavView {
    fn get_description(&self) -> String {
        String::from("compact navigation viewer")
    }
}

pub struct RelaxedNavView;

impl NavView for RelaxedNavView {
    fn get_description(&self) -> String {
        String::from("relaxed navigation viewer")
    }
}