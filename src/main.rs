mod components;
mod resources;
mod systems;
mod utils;
mod traits;
mod constants;
pub mod units;

use bevy::prelude::*;
use bevy::window::Window;
use components::*;
use resources::*;
use systems::*;
pub use traits::*;
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hex Grid Game".to_string(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SelectionSystemPlugin)  // Add the selection plugin
        .init_resource::<CameraConfig>()
        .init_resource::<ContextMenuState>()  // Still needed for the new context menu
        .init_resource::<DetailedMenuState>()
        .init_resource::<SelectionState>()  // Initialize SelectionState
        .init_resource::<MouseState>()      // Initialize MouseState
        .add_event::<UnitCommand>()
        .add_systems(Startup, (
            setup_camera,
            draw_hex_grid,
            entity_startup_system,
        ))
        .add_systems(Update, (
            // Camera systems
            (
                camera_movement_system,
                mouse_camera_movement_system,
                camera_zoom_system,
                camera_angle_system,
                animate_units_system,
            ),
            // Post-selection systems (selection is handled by SelectionPlugin)
            (
                hex_sprite_system,
                entity_movement_system,
                unit_command_system
            ),
        ))
        
        .run();
}

// Keep your existing GameSet if you want to use it for other systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSet {
    Camera,
    Selection,
    Gameplay,
}