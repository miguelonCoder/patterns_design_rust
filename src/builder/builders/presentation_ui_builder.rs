use crate::builder::{
    builders::ui_builder::UiBuilder,
    ui::{Ui, UiStyle},
    components::{
        bottom_indicators::{BottomIndicator, CompactBottomIndicator},
        nav_view::{CompactNavView, NavView},
    },
};
#[derive(Default)]
pub struct PresentationUiBuilder {
    nav_view: Option<Box<dyn NavView>>,
    bottom_indicator: Option<Box<dyn BottomIndicator>>,
}

impl UiBuilder for PresentationUiBuilder {

    fn set_bottom_indicators(&mut self, bottom_indicators: Option<Box<dyn BottomIndicator>>) {
        self.bottom_indicator = bottom_indicators;
    }

    fn set_nav_view(&mut self, nav_view: Option<Box<dyn NavView>>) {
        self.nav_view = nav_view;
    }

    fn build(self) -> Ui {
        Ui::render(
            None,
            None,
            self.nav_view.expect("Especifica el componente de visor de navegación"),
            None,
            self.bottom_indicator.expect("Especifica el componente de cinta inferior de indicaciones"),
            UiStyle::Compact,
        )
    }
}