
pub trait SidePanel {
    fn get_description(&self) -> String;

}

pub struct CompactSidePanel;

impl SidePanel for CompactSidePanel {
    fn get_description(&self) -> String {
        String::from("compact side panel")
    }
}

pub struct RelaxedSidePanel;

impl SidePanel for RelaxedSidePanel {
    fn get_description(&self) -> String {
        String::from("relaxed side panel")
    }
}