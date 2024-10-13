
pub trait BottomIndicator {
    fn get_description(&self) -> String;

}

pub struct CompactBottomIndicator;

impl BottomIndicator for CompactBottomIndicator {

    fn get_description(&self) -> String {
        String::from("compact Bottom Indicator")
    }
}

pub struct RelaxedBottomIndicator;

impl BottomIndicator for RelaxedBottomIndicator {
    fn get_description(&self) -> String {
        String::from("relaxed Bottom Indicator")
    }
}