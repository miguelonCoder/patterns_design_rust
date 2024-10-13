
pub trait SideOptions {
    fn get_description(&self) -> String;

}

pub struct CompactSideOptions;

impl SideOptions for CompactSideOptions {
    fn get_description(&self) -> String {
        String::from("compact side options")
    }
}

pub struct RelaxedSideOptions;

impl SideOptions for RelaxedSideOptions {
    fn get_description(&self) -> String {
        String::from("relaxed side options")
    }
}