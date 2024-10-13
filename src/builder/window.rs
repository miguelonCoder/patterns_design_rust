use crate::builder::builders::relaxed_ui_builder::RelaxedUiBuilder;
use crate::builder::builders::ui_builder::UiBuilder;
use crate::builder::components::bottom_indicators::{CompactBottomIndicator, RelaxedBottomIndicator};
use crate::builder::components::nav_view::{CompactNavView, RelaxedNavView};
use crate::builder::components::ribbon::{CompactRibbon, RelaxedRibbon};
use crate::builder::components::side_options::{CompactSideOptions, RelaxedSideOptions};
use crate::builder::components::side_panel::{CompactSidePanel, RelaxedSidePanel};
use crate::builder::ui::Ui;

pub struct Window;
impl Window {
    pub fn configure_relaxed_view(builder: &mut Box<impl UiBuilder>){
        builder.set_bottom_indicators(Some(Box::new(RelaxedBottomIndicator)));
        builder.set_nav_view(Some(Box::new(RelaxedNavView)));
        builder.set_ribbon(Some(Box::new(RelaxedRibbon)));
        builder.set_side_options(Some(Box::new(RelaxedSideOptions)));
        builder.set_side_panel(Some(Box::new(RelaxedSidePanel)));
    }

    pub fn configure_compact_view(builder: &mut Box<impl UiBuilder>){
        builder.set_bottom_indicators(Some(Box::new(CompactBottomIndicator)));
        builder.set_nav_view(Some(Box::new(CompactNavView)));
        builder.set_ribbon(Some(Box::new(CompactRibbon)));
        builder.set_side_options(Some(Box::new(CompactSideOptions)));
        builder.set_side_panel(Some(Box::new(CompactSidePanel)));
    }

    pub fn configure_presentation_view(builder: &mut Box<impl UiBuilder>){
        builder.set_bottom_indicators(Some(Box::new(CompactBottomIndicator)));
        builder.set_nav_view(Some(Box::new(CompactNavView)));
    }
}