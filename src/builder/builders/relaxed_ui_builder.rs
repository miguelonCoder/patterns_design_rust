use crate::builder::{
    builders::ui_builder::UiBuilder,
    ui::{Ui, UiStyle},
    components::{
        bottom_indicators::{BottomIndicator, RelaxedBottomIndicator},
        nav_view::{RelaxedNavView, NavView},
        ribbon::{RelaxedRibbon, Ribbon},
        side_options::{RelaxedSideOptions, SideOptions},
        side_panel::{RelaxedSidePanel, SidePanel},
    },
};
#[derive(Default)]
pub struct RelaxedUiBuilder {
    side_panel: Option<Box<dyn SidePanel>>,
    side_options: Option<Box<dyn SideOptions>>,
    nav_view: Option<Box<dyn NavView>>,
    ribbon: Option<Box<dyn Ribbon>>,
    bottom_indicator: Option<Box<dyn BottomIndicator>>,
}

impl UiBuilder for RelaxedUiBuilder {
    fn set_ribbon(&mut self, ribbon: Option<Box<dyn Ribbon>>) {
        self.ribbon = ribbon;
    }

    fn set_side_panel(&mut self, side_panel: Option<Box<dyn SidePanel>>) {
        self.side_panel = side_panel;
    }
    
    fn set_side_options(&mut self, side_options: Option<Box<dyn SideOptions>>) {
        self.side_options = side_options;
    }

    fn set_bottom_indicators(&mut self, bottom_indicators: Option<Box<dyn BottomIndicator>>) {
        self.bottom_indicator = bottom_indicators;
    }

    fn set_nav_view(&mut self, nav_view: Option<Box<dyn NavView>>) {
        self.nav_view = nav_view;
    }

    fn build(self) -> Ui {
        Ui::render(
            self.side_panel,
            self.side_options,
            self.nav_view.expect("Especifica el componente de visor de navegación"),
            self.ribbon,
            self.bottom_indicator.expect("Especifica el componente de cinta inferior de indicaciones"),
            UiStyle::Relaxed
        )
    }
}