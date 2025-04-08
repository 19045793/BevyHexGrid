use bevy::ui::Style;
use bevy::{prelude::*, utils::HashMap};

use crate::components::*;
use crate::constants::*;
use crate::utils::axial_to_world;

#[derive(Resource)]
pub struct MouseState {
    pub pressed: bool,
    pub last_position: Vec2,
    pub is_dragging: bool,
    pub was_drag: bool,
    pub drag_button: Option<MouseButton>,
    pub drag_threshold: f32,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            pressed: false,
            last_position: Vec2::ZERO,
            is_dragging: false,
            was_drag: false,
            drag_button: None,
            drag_threshold: 5.0, // Adjust this value to change sensitivity
        }
    }

    pub fn start_press(&mut self, position: Vec2, button: MouseButton) {
        self.pressed = true;
        self.last_position = position;
        self.is_dragging = false;
        self.was_drag = false;
        self.drag_button = Some(button);
    }

    pub fn end_press(&mut self) {
        self.pressed = false;
        self.is_dragging = false;
        self.drag_button = None;
        // Note: we keep was_drag flag until it's explicitly reset
    }

    pub fn reset_drag_state(&mut self) {
        self.was_drag = false;
    }

    pub fn check_drag(&mut self, current_position: Vec2) -> bool {
        if self.pressed {
            let delta = current_position - self.last_position;
            if delta.length() > self.drag_threshold {
                self.is_dragging = true;
                self.was_drag = true;
                return true;
            }
        }
        self.is_dragging
    }

    pub fn update_position(&mut self, position: Vec2) {
        self.last_position = position;
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}



#[derive(Resource)]
pub struct TerrainTextureSet {
    pub terrains: Vec<Terrain>,
    pub texture_variants: HashMap<String, Vec<Handle<Image>>>,
}

#[derive(Resource, Clone)]
pub struct UnitTextureSet {
    pub animation_sets: HashMap<String, AnimationSet>,
}
pub struct UnitSpriteData {
    pub name: String,
    pub animation_handles: AnimationSet,
}

#[derive(Resource, Default)]
pub struct DetailedMenuState {
    pub is_open: bool,
    pub unit_entity: Option<Entity>,
    pub position: Vec2,
}

#[derive(Resource, Default)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position: Vec2,
    pub target_entity: Option<Entity>,
    pub entity_type: Option<SelectableType>,  // Add entity type
    pub entity_data: Option<Box<dyn std::any::Any + Send + Sync>>, // Add Send + Sync bounds
}


#[derive(Resource)]
pub struct SelectionState {
    // Drag state
    pub is_dragging: bool,
    pub drag_start_world_pos: Vec2,
    pub drag_current_world_pos: Vec2,
    pub drag_start_screen_pos: Vec2,
    
    // Selection state
    pub selected_entity: Option<Entity>,
    pub selected_entities: Vec<Entity>,  // Added for multiple selection
    pub hovered_entity: Option<Entity>,
    
    // Hover check state
    pub hover_check_needed: bool,
    pub hover_check_pos: Vec2,
    
    // Context menu state
    pub context_menu_trigger_pos: Option<(Vec2, Vec2)>, // (world_pos, screen_pos)
    
    // Multi-select state
    pub last_click_time: f32,
    pub last_clicked_pos: Option<Vec2>,
    pub last_clicked_entity: Option<Entity>,
}

impl SelectionState {
    // Keep your existing methods...
    pub fn start_drag(&mut self, screen_pos: Vec2) {
        self.is_dragging = true;
        self.drag_start_screen_pos = screen_pos;
        self.drag_start_world_pos = Vec2::ZERO; // Will be updated in update_drag
        self.clear_hover();
        }
        
        pub fn update_drag(&mut self, screen_pos: Vec2, camera: &Camera, camera_transform: &GlobalTransform) {
            if self.is_dragging {
                if let Some(world_pos) = camera.viewport_to_world(camera_transform, screen_pos) {
                    self.drag_current_world_pos = world_pos.origin.truncate();
                }
            }
        }
        
        pub fn end_drag(&mut self) {
            self.is_dragging = false;
            self.hover_check_needed = true;
        }
        
        pub fn is_active(&self) -> bool {
            self.is_dragging || self.hover_check_needed
        }
        
        pub fn get_selection_rect(&self) -> Rect {
            if self.is_dragging {
                Rect::from_corners(self.drag_start_world_pos, self.drag_current_world_pos)
            } else {
                Rect::default()
            }
        }
        
        pub fn should_hover(&self, pos: Vec2) -> bool {
            if self.is_dragging {
                self.get_selection_rect().contains(pos)
            } else {
                (pos - self.hover_check_pos).length_squared() < HOVER_THRESHOLD_SQUARED
            }
        }
        
        pub fn clear_hover(&mut self) {
            self.hovered_entity = None;
            self.hover_check_needed = false;
        }
        
        pub fn trigger_context_menu(&mut self, world_pos: Vec2, screen_pos: Vec2) {
            self.context_menu_trigger_pos = Some((world_pos, screen_pos));
        }
        
        pub fn select_entity(&mut self, entity: Entity) {
            self.selected_entity = Some(entity);
        }
        
        pub fn deselect(&mut self) {
            self.selected_entity = None;
        }
        
        pub fn set_hover(&mut self, entity: Option<Entity>) {
            self.hovered_entity = entity;
            self.hover_check_needed = entity.is_none();
        }

    // Add these new methods for multiple selection
    pub fn handle_selection(&mut self, entity: Entity, shift_held: bool, time: f32, pos: Vec2) {
        let is_double_click = self.check_double_click(time, pos, entity);
        
        if shift_held {
            // Shift-click: Add to or remove from multiple selection
            if self.selected_entities.contains(&entity) {
                self.selected_entities.retain(|&e| e != entity);
            } else {
                self.selected_entities.push(entity);
            }
            // Maintain primary selection
            self.selected_entity = Some(entity);
        } else if is_double_click {
            // Double-click: Select all similar entities at the same position
            self.selected_entity = Some(entity);
            if !self.selected_entities.contains(&entity) {
                self.selected_entities.push(entity);
            }
        } else {
            // Single click: Clear multiple selection and select single entity
            self.selected_entities.clear();
            self.selected_entity = Some(entity);
            self.selected_entities.push(entity);
        }

        // Update click tracking
        self.last_click_time = time;
        self.last_clicked_pos = Some(pos);
        self.last_clicked_entity = Some(entity);
    }

    fn check_double_click(&self, current_time: f32, current_pos: Vec2, entity: Entity) -> bool {
        const DOUBLE_CLICK_TIME: f32 = 0.5; // Half a second for double-click
        const DOUBLE_CLICK_DISTANCE: f32 = 5.0; // Pixels of movement allowed between clicks

        if let (Some(last_pos), Some(last_entity)) = (self.last_clicked_pos, self.last_clicked_entity) {
            let time_diff = current_time - self.last_click_time;
            let distance = (current_pos - last_pos).length();
            
            time_diff < DOUBLE_CLICK_TIME 
                && distance < DOUBLE_CLICK_DISTANCE 
                && last_entity == entity
        } else {
            false
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_entity = None;
        self.selected_entities.clear();
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected_entities.contains(&entity) || self.selected_entity == Some(entity)
    }

    pub fn get_selected_count(&self) -> usize {
        self.selected_entities.len()
    }

    pub fn get_primary_selection(&self) -> Option<Entity> {
        self.selected_entity
    }

    pub fn get_all_selections(&self) -> &[Entity] {
        &self.selected_entities
    }
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_start_world_pos: Vec2::ZERO,
            drag_current_world_pos: Vec2::ZERO,
            drag_start_screen_pos: Vec2::ZERO,
            selected_entity: None,
            selected_entities: Vec::new(),
            hovered_entity: None,
            hover_check_needed: false,
            hover_check_pos: Vec2::ZERO,
            context_menu_trigger_pos: None,
            last_click_time: 0.0,
            last_clicked_pos: None,
            last_clicked_entity: None,
        }
    }
}

#[derive(Resource)]
pub struct SelectionConfig {
    pub drag_threshold: f32,
    pub tile_highlight_color: Color,
    pub hover_highlight_color: Color,
    pub selection_highlight_color: Color,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            drag_threshold: 5.0,
            tile_highlight_color: Color::rgba(1.0, 1.0, 1.0, 0.3),
            hover_highlight_color: Color::rgba(0.8, 0.8, 1.0, 0.4),
            selection_highlight_color: Color::rgba(0.5, 0.8, 1.0, 0.5),
        }
    }
}

// Selection feedback event
#[derive(Event)]
pub enum SelectionEvent {
    Selected(Entity),
    Deselected(Entity),
    HoverStart(Entity),
    HoverEnd(Entity),
}