use crate::builder::components::bottom_indicators::BottomIndicator;
use crate::builder::components::nav_view::NavView;
use crate::builder::components::ribbon::Ribbon;
use crate::builder::components::side_options::SideOptions;
use crate::builder::components::side_panel::SidePanel;
use crate::builder::ui::Ui;

pub trait UiBuilder {
    fn set_ribbon(&mut self, ribbon: Option<Box<dyn Ribbon>>){
    }
    fn set_side_panel(&mut self, side_panel: Option<Box<dyn SidePanel>>){
    }

    fn set_side_options(&mut self, side_options: Option<Box<dyn SideOptions>>) {
    }

    fn set_bottom_indicators(&mut self, bottom_indicators: Option<Box<dyn BottomIndicator>>);

    fn set_nav_view(&mut self, nav_view : Option<Box<dyn NavView>>);

    fn build(self) -> Ui;
}