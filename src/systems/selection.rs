use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::{Selectable, SelectableType};
use crate::resources::MouseState;

use super::MainCamera;

// Resource to track selection state
#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub selection_type: Option<SelectableType>,
}

// Plugin to register all selection-related systems
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectionState>()
            .init_resource::<MouseState>()
            .add_systems(Update, (
                selection_hover_system,
                selection_click_system,
                selection_highlight_system,
            ));
    }
}

// System that handles hovering over selectable entities
fn selection_hover_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut selectables_query: Query<(Entity, &GlobalTransform, &mut Selectable)>,
    mut selection_state: ResMut<SelectionState>,
) {
    // Reset previous hover state
    let mut closest_entity = None;
    let mut closest_distance = f32::MAX;
    let mut closest_depth = f32::MAX;

    // Get the camera and window
    let window = match windows.get_single() {
        Ok(win) => win,
        Err(_) => return,
    };

    let (camera, camera_transform) = match camera_q.get_single() {
        Ok(cam) => cam,
        Err(_) => return,
    };

    if let Some(cursor_position) = window.cursor_position() {
        // Loop through all selectables to find what's under the cursor
        for (entity, transform, _selectable) in selectables_query.iter_mut() {
            // Project entity position to screen space
            let entity_position = transform.translation();
            
            // First check if we can see this point from the camera
            if let Some(viewport_pos) = camera.world_to_viewport(camera_transform, entity_position) {
                // Calculate distance from cursor to entity in screen space
                let distance = viewport_pos.distance(cursor_position);
                
                // We use depth (z-value) as a tiebreaker for entities at the same screen position
                if distance < 50.0 && (distance < closest_distance || 
                   (distance == closest_distance && entity_position.z < closest_depth)) {
                    closest_entity = Some(entity);
                    closest_distance = distance;
                    closest_depth = entity_position.z;
                }
            }
        }
    }

    // Update hover state
    for (entity, _, mut selectable) in selectables_query.iter_mut() {
        selectable.is_hovered = Some(entity) == closest_entity;
        
        if selectable.is_hovered && selection_state.hovered_entity != Some(entity) {
            selection_state.hovered_entity = Some(entity);
        }
    }
    
    // Clear hover state if nothing is hovered
    if closest_entity.is_none() {
        selection_state.hovered_entity = None;
    }
}

// System that handles selection when clicking on entities
fn selection_click_system(
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut selectables_query: Query<(Entity, &GlobalTransform, &mut Selectable)>,
    mut selection_state: ResMut<SelectionState>,
) {
    let window = match windows.get_single() {
        Ok(win) => win,
        Err(_) => return,
    };

    let (camera, camera_transform) = match camera_q.get_single() {
        Ok(cam) => cam,
        Err(_) => return,
    };

    if let Some(cursor_position) = window.cursor_position() {
        // Update mouse state
        if mouse_buttons.just_pressed(MouseButton::Left) {
            mouse_state.start_press(cursor_position, MouseButton::Left);
        }

        // Check for mouse drag
        if mouse_state.pressed {
            mouse_state.check_drag(cursor_position);
        }

        // Handle click (not drag) release
        if mouse_buttons.just_released(MouseButton::Left) && !mouse_state.was_drag {
            // First find the entity under the cursor
            let mut closest_entity = None;
            let mut closest_distance = f32::MAX;
            let mut closest_depth = f32::MAX;
            let mut selection_type = None;

            // Raycast from cursor position to world
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                // Look for the closest entity to our cursor
                for (entity, transform, selectable) in selectables_query.iter_mut() {
                    let entity_position = transform.translation();
                    
                    // Project entity to screen space
                    if let Some(screen_pos) = camera.world_to_viewport(camera_transform, entity_position) {
                        let distance = screen_pos.distance(cursor_position);
                        
                        // Use selection radius based on the type of selectable
                        let selection_radius = match selectable.selectable_type {
                            SelectableType::Unit => 40.0,
                            SelectableType::Tile => 30.0,
                            SelectableType::Building => 35.0,
                            SelectableType::UI => 15.0,
                        };
                        
                        if distance < selection_radius && 
                           (distance < closest_distance || 
                            (distance == closest_distance && entity_position.z < closest_depth)) {
                            closest_entity = Some(entity);
                            closest_distance = distance;
                            closest_depth = entity_position.z;
                            selection_type = Some(selectable.selectable_type);
                        }
                    }
                }
            }

            // Update selection state for all entities
            for (entity, _, mut selectable) in selectables_query.iter_mut() {
                // Only select the closest entity
                let is_selected = Some(entity) == closest_entity;
                selectable.is_selected = is_selected;
                
                // Update the global selection state
                if is_selected {
                    selection_state.selected_entity = Some(entity);
                    selection_state.selection_type = selection_type;
                }
            }
            
            // If nothing was clicked, clear selection
            if closest_entity.is_none() {
                selection_state.selected_entity = None;
                selection_state.selection_type = None;
            }
        }
        
        // Reset mouse state after release
        if mouse_buttons.just_released(MouseButton::Left) {
            mouse_state.end_press();
            mouse_state.reset_drag_state();
        }
    }
}

// Updated system to apply highlighting directly to the entity's sprite
fn selection_highlight_system(
    mut query: Query<(&Selectable, &mut Sprite)>,
) {
    for (selectable, mut sprite) in query.iter_mut() {
        // Apply visual changes based on selection state
        if selectable.is_selected {
            // Selected state - apply a bright highlight effect
            sprite.color = Color::rgba(1.0, 1.0, 0.6, 1.0); // Bright highlight
        } else if selectable.is_hovered {
            // Hover state - subtle highlight
            sprite.color = Color::rgba(0.9, 0.9, 1.0, 1.0); // Light blue highlight
        } else {
            // Default state
            sprite.color = Color::WHITE;
        }
    }
}