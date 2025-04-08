use bevy::prelude::*;

use bevy::ui::Style;

use crate::resources::UnitTextureSet;
use crate::utils::axial_to_world;
use crate::constants::HEX_SIZE;


#[derive(Component, Clone)]
pub struct Terrain {
    pub name: String,
    pub texture_handles: Vec<Handle<Image>>,
}

#[derive(Component)]
pub struct SelectionVisualization;

#[derive(Component)]
pub struct ContextMenuItem;

// New component for the "More Details" button
#[derive(Component)]
pub struct UnitInfoButton;

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub id: i32,
    pub q: i32,
    pub r: i32,
    pub position: Vec3,
    pub terrain: String,
    pub texture_variant: usize,  // Just store the index
}

#[derive(Component)]
pub struct HexSprite {
    pub orientation: f32,
}

#[derive(Component)]
pub struct UnitState {
    pub is_moving: bool,
    pub target_position: Option<(i32, i32)>,
}


#[derive(Component, Debug)]
pub enum EntityType {
    Unit,
    Building,
    Item,
}

#[derive(Component, Debug)]
pub struct GameEntity {
    pub name: String,
    pub description: String,
    pub entity_type: EntityType,
    pub position: (i32, i32),
    pub sprite: Handle<Image>,
}

#[derive(Component, Debug)]
pub struct Unit {
    pub entity: GameEntity,
    pub health: f32,
    pub movement_range: i32,
    pub attack_damage: f32,
}
#[derive(Component)]
pub enum UnitAnimation {
    Idle,
    Walking,
    Attacking,
}
#[derive(Component)]
pub struct AnimationData {
    pub frames: Vec<Handle<Image>>,
    pub current_frame: usize,
    pub timer: Timer,
    pub is_looping: bool,
}

#[derive(Component)]
pub struct AnimationState {
    pub idle: AnimationData,
    pub walking: AnimationData,
    pub attacking: AnimationData,
    pub current_animation: UnitAnimation,
}

#[derive(PartialEq, Clone)]
pub struct AnimationSet {
    pub idle: Option<Vec<Handle<Image>>>,
    pub walk: Option<Vec<Handle<Image>>>,
    pub attack: Option<Vec<Handle<Image>>>,
}
impl Default for AnimationSet {
    fn default() -> Self {
        Self {
            idle: None,
            walk: None,
            attack: None,
        }
    }
}

#[derive(Component, Debug)]
pub struct Selected;

#[derive(Component, Debug)]
pub struct Hovered;

#[derive(Component)]
pub struct SelectionOutline;

#[derive(Component, Clone, Debug)]
pub enum UnitType {
    Warrior,
    Archer,
    Mage,
    ElvenArcher
}

impl UnitType {
    pub fn name(&self) -> &'static str {
        match self {
            UnitType::Warrior => "Warrior",
            UnitType::Archer => "Archer",
            UnitType::Mage => "Mage",
            UnitType::ElvenArcher => "Elven Archer",
        }
    }
}
pub struct UnitConfig {
    pub name: &'static str,
    pub idle_frames: usize,
    pub walk_frames: usize,
    pub attack_frames: usize,
    pub animation_timings: AnimationTimings,
    pub scale: Vec3,
}

pub struct AnimationTimings {
    pub idle: f32,
    pub walk: f32,
    pub attack: f32,
}



// UI Components
#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct PlayerInfoPanel;

#[derive(Component)]
pub struct UnitInfoPanel;

#[derive(Component)]
pub struct UnitInfoText;

#[derive(Component)]
pub struct DetailedMenuText;
// New components for the detailed menu
#[derive(Component)]
pub struct DetailedMenu;

#[derive(Component)]
pub struct DetailedMenuButton;

#[derive(Component)]
pub struct UiBlocking; // Marker component for UI elements that block world selection// components.rs

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectableType {
    Unit,
    Tile, 
    Building,
    UI,
}

impl Default for SelectableType {
    fn default() -> Self {
        SelectableType::Unit
    }
}

// Component for entities that can be selected
#[derive(Component, Default)]
pub struct Selectable {
    pub is_selected: bool,
    pub is_hovered: bool,
    pub selectable_type: SelectableType,
}

impl Selectable {
    pub fn new(selectable_type: SelectableType) -> Self {
        Self {
            is_selected: false,
            is_hovered: false,
            selectable_type,
        }
    }
}

// You can also add a visual component for selection indicators
#[derive(Component)]
pub struct SelectionIndicator;