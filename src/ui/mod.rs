// ui/mod.rs
pub(crate) mod root;
pub(crate) mod menu;

use bevy::prelude::*;
pub use root::*;
pub use menu::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_system)
            .add_systems(Update, (
                update_unit_info_system,
                handle_context_menu,
                handle_detailed_menu,
                handle_menu_interaction,
            ).chain());
    }
}