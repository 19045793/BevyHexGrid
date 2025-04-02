use bevy::prelude::*;

use crate::components::Selectable;

#[derive(Resource, Default, Debug)]
pub struct SelectionState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: Vec2,
    pub target_entity: Option<Entity>,
}

#[derive(Resource)]
pub struct MouseState {
    pub pressed: bool,
    pub last_position: Vec2,
    pub is_dragging: bool,
    pub was_drag: bool,
    pub drag_button: Option<MouseButton>,
}


impl MouseState {
    pub fn new() -> Self {
        Self {
            pressed: false,
            last_position: Vec2::ZERO,
            is_dragging: false,
            was_drag: false,
            drag_button: None,
        }
    }

    pub fn start_press(&mut self, position: Vec2, button: MouseButton) {
        self.pressed = true;
        self.last_position = position;
        self.is_dragging = false;
        self.was_drag = false;
        self.drag_button = Some(button);
    }

    pub fn end_press(&mut self, button: MouseButton) {
        if self.drag_button == Some(button) {
            self.pressed = false;
            self.is_dragging = false;
            // Keep was_drag if we were dragging
            println!("End press, was_drag: {}", self.was_drag); // Debug print
            self.drag_button = None;
        }
    }

    // Add new method to reset drag state after selection system has run
    pub fn reset_drag_state(&mut self) {
        self.was_drag = false;
    }

    pub fn check_drag(&mut self, current_position: Vec2) -> bool {
        if self.pressed {
            let delta = current_position - self.last_position;
            if delta.length() > 1.0 {
                self.is_dragging = true;
                self.was_drag = true;
                println!("Started dragging"); // Debug print
            }
        }
        self.is_dragging
    }

    pub fn update_position(&mut self, position: Vec2) {
        self.last_position = position;
    }

    pub fn was_dragging(&self, button: MouseButton) -> bool {
        let result = self.was_drag && self.drag_button == Some(button);
        println!("Checking was_dragging: {} (was_drag: {}, button match: {})", 
            result, self.was_drag, self.drag_button == Some(button)); // Debug print
        result
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            pressed: false,
            last_position: Vec2::ZERO,
            is_dragging: false,
            was_drag: false,
            drag_button: None,
        }
    }
}


fn selection_system(
    mut mouse_state: ResMut<MouseState>,
    mut selection_state: ResMut<SelectionState>,
    mouse_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_selectable: Query<(Entity, &Transform, &mut Selectable)>,
) {
    let window = window.single();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_position) = window.cursor_position() {
        mouse_state.update_position(cursor_position);

        if mouse_input.just_pressed(MouseButton::Left) {
            mouse_state.start_press(cursor_position, MouseButton::Left);
        }

        if mouse_input.just_released(MouseButton::Left) {
            if !mouse_state.was_dragging(MouseButton::Left) {
                for (entity, transform, mut selectable) in q_selectable.iter_mut() {
                    // Convert cursor screen position to world position
                    if let Some(world_cursor_pos) = camera.viewport_to_world(camera_transform, cursor_position)
                        .map(|ray| ray.origin.truncate()) {
                        
                        // Use the entity's world position directly
                        let entity_world_pos = transform.translation.truncate();
                        
                        // Calculate distance in world space
                        let distance = entity_world_pos.distance(world_cursor_pos);
                        
                        if distance < 10.0 { // Adjust selection radius as needed
                            selectable.is_selected = true;
                            selection_state.selected_entity = Some(entity);
                            break;
                        } else {
                            selectable.is_selected = false;
                        }
                    }
                }
            }
            mouse_state.end_press(MouseButton::Left);
        }

        if mouse_state.check_drag(cursor_position) {
            // Handle drag logic if needed
        }
    }
}

// In your systems file
fn hover_system(
    mut q_selectable: Query<(Entity, &Transform, &mut Selectable)>,
    window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_state: Res<MouseState>,
    mut selection_state: ResMut<SelectionState>,
) {
    let window = window.single();
    let (camera, camera_transform) = q_camera.single();

    if let Some(cursor_position) = window.cursor_position() {
        // Reset previous hover state
        for (_, _, mut selectable) in q_selectable.iter_mut() {
            selectable.is_hovered = false;
        }

        // Check for hover
        for (entity, transform, mut selectable) in q_selectable.iter_mut() {
            if let Some(world_cursor_pos) = camera.viewport_to_world(camera_transform, cursor_position)
                .map(|ray| ray.origin.truncate()) {
                
                let entity_world_pos = transform.translation.truncate();
                let distance = entity_world_pos.distance(world_cursor_pos);
                
                if distance < 10.0 { // Adjust selection radius as needed
                    selectable.is_hovered = true;
                    selection_state.hovered_entity = Some(entity);
                    break;
                }
            }
        }
    }
}

fn highlight_system(
    mut q_selectable: Query<(&Selectable, &mut Sprite)>,
) {
    for (selectable, mut sprite) in q_selectable.iter_mut() {
        if selectable.is_selected {
            // Highlight selected entities (e.g., change color)
            sprite.color = Color::GOLD;
        } else if selectable.is_hovered {
            // Highlight hovered entities (e.g., change color)
            sprite.color = Color::SILVER;
        } else {
            // Reset to default color
            sprite.color = Color::WHITE;
        }
    }
}
pub struct SelectionSystemPlugin;

impl Plugin for SelectionSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectionState>()
            .init_resource::<MouseState>()
            .add_systems(Update, (
                selection_system,
                hover_system,
                highlight_system));
    }
}