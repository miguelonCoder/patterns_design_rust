
pub trait Ribbon {
    fn get_description(&self) -> String;

}

pub struct CompactRibbon;

impl Ribbon for CompactRibbon {
    fn get_description(&self) -> String {
        String::from("compact ribbon")
    }
}

pub struct RelaxedRibbon;

impl Ribbon for RelaxedRibbon {
    fn get_description(&self) -> String {
        String::from("relaxed ribbon")
    }
}