use crate::builder::components::bottom_indicators::BottomIndicator;
use crate::builder::components::nav_view::NavView;
use crate::builder::components::ribbon::Ribbon;
use crate::builder::components::side_options::SideOptions;
use crate::builder::components::side_panel::SidePanel;

pub enum UiStyle {
    Relaxed,
    Compact,
    Presentation
}

pub struct Ui {
    side_panel: Option<Box<dyn SidePanel>>,
    side_options: Option<Box<dyn SideOptions>>,
    nav_view: Box<dyn NavView>,
    ribbon: Option<Box<dyn Ribbon>>,
    bottom_indicator: Box<dyn BottomIndicator>,
    style: UiStyle
}

impl Ui {

    pub fn render(
        side_panel: Option<Box<dyn SidePanel>>,
        side_options: Option<Box<dyn SideOptions>>,
        nav_view: Box<dyn NavView>,
        ribbon: Option<Box<dyn Ribbon>>,
        bottom_indicator: Box<dyn BottomIndicator>,
        style: UiStyle
    ) -> Self {
      Self {
          side_panel,
          side_options,
          nav_view,
          ribbon,
          bottom_indicator,
          style
      }
    }

    pub fn get_ui_definition(&self) -> String {
        let without_content = String::from("Sin contenido");

        let nav_view_content = &self.nav_view.get_description();

        let ribbon_content = match &self.ribbon {
            Some(component) => &component.get_description(),
            None => &without_content
        };

        let side_options_content = match &self.side_options {
            Some(component) => &component.get_description(),
            None => &without_content
        };

        let side_panel_content = match &self.side_panel {
            Some(component) => &component.get_description(),
            None => &without_content
        };

        let bottom_indicator_content = &self.bottom_indicator.get_description();


        format!(r#"Estilo de la ui
        Arriba: {ribbon_content}
        Abajo: {bottom_indicator_content}
        Centro: {nav_view_content}
        Derecha: {side_options_content}
        Izquierda: {side_panel_content}
        "#, )

    }


}