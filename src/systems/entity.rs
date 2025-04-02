use bevy::{prelude::*, utils::HashMap};
use crate::components::{Unit, UnitType, AnimationData, AnimationSet, AnimationState, EntityType, GameEntity, Selectable, SelectableType, UnitAnimation, UnitState};
use crate::constants::UNIT_Z;
use crate::units::*;
use crate::resources::UnitTextureSet;
use crate::traits::Moveable;
use crate::utils::coordinates::axial_to_world;

use bevy::ui::Style;

// Constants for unit configuration
const DEFAULT_UNIT_HEALTH: f32 = 100.0;
const DEFAULT_MOVEMENT_RANGE: i32 = 2;
const DEFAULT_ATTACK_DAMAGE: f32 = 10.0;

impl Default for UnitState {
    fn default() -> Self {
        Self {
            is_moving: false,
            target_position: None,
        }
    }
}

pub fn entity_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut animation_sets = HashMap::new();

    // Initialize unit-specific animations
    animation_sets.insert(
        "Warrior".to_string(),
        Warrior::initialize(&asset_server)
    );
    animation_sets.insert(
        "Archer".to_string(),
        Archer::initialize(&asset_server)
    );
    animation_sets.insert(
        "Elven Archer".to_string(),
        ElvenArcher::initialize(&asset_server)
    );
    
    // ***spare debug implementation***

    //let warrior_set = Warrior::initialize(&asset_server);
    //println!("Warrior: idle={:?}, walk={:?}, attack={:?}", 
    //    warrior_set.idle.as_ref().map(|v| v.len()),
    //    warrior_set.walk.as_ref().map(|v| v.len()),
    //    warrior_set.attack.as_ref().map(|v| v.len())
    //);
    //animation_sets.insert("Warrior".to_string(), warrior_set);

    // Create and insert the resource
    let unit_textures = UnitTextureSet {
        animation_sets,
    };
    commands.insert_resource(unit_textures.clone());

    // Spawn initial units
    let initial_units = vec![
        (UnitType::Warrior, 0, 0),
        (UnitType::Archer, 1, 1),
        (UnitType::ElvenArcher, 1, 0),
    ];

    for (unit_type, q, r) in initial_units {
        match unit_type {
            UnitType::Warrior => spawn_unit::<Warrior>(&mut commands, &unit_textures, unit_type, q, r),
            UnitType::Archer => spawn_unit::<Archer>(&mut commands, &unit_textures, unit_type, q, r),
            UnitType::ElvenArcher => spawn_unit::<ElvenArcher>(&mut commands, &unit_textures, unit_type, q, r),
            UnitType::Mage => todo!("Implement Mage spawning"),
        };
    }
}
// Keep the spawn_unit function separate for reuse
// In entity.rs
pub fn spawn_unit<T: UnitBehavior>(
    commands: &mut Commands,
    unit_textures: &UnitTextureSet,
    unit_type: UnitType,
    q: i32,
    r: i32,
) -> Entity {
    if let Some(animations) = unit_textures.animation_sets.get(unit_type.name()) {
        let default_texture = animations.idle.as_ref()
            .and_then(|frames| frames.first())
            .or_else(|| animations.walk.as_ref().and_then(|frames| frames.first()))
            .or_else(|| animations.attack.as_ref().and_then(|frames| frames.first()))
            .expect("No animation frames found for unit")
            .clone();

        let base_unit = BaseUnit::new(
            unit_type.name().to_string(),
            default_texture.clone(),
            q,
            r,
        );

        let unit = T::new(base_unit);
        
        // Create the Unit component with the base unit data
        let unit_component = Unit {
            entity: GameEntity {
                name: unit_type.name().to_string(),
                description: "A unit".to_string(),
                entity_type: EntityType::Unit,
                position: (q, r),
                sprite: default_texture.clone(),
            },
            health: DEFAULT_UNIT_HEALTH,
            movement_range: DEFAULT_MOVEMENT_RANGE,
            attack_damage: DEFAULT_ATTACK_DAMAGE,
        };

        commands.spawn((
            unit,  // The unit-specific type (e.g., ElvenArcher)
            unit_component,  // Add the Unit component (which implements Selectable)
            unit_type.clone(),  // Add the UnitType component
            SpriteBundle {
                texture: default_texture,
                transform: Transform::from_translation(Vec3::new(
                    axial_to_world(q, r).x,
                    axial_to_world(q, r).y,
                    UNIT_Z
                )).with_scale(T::get_scale()),
                ..default()
            },
            T::create_animation_state(animations),
            UnitState::default(),
            Selectable::default(),  // Add the Selectable component
            Name::new(format!("Unit: {}", unit_type.name())),
        )).id()
    } else {
        panic!("Failed to load animations for unit type: {:?}", unit_type);
    }
}
pub struct UnitBundle {
    pub unit: Unit,  // Add this
    pub sprite_bundle: SpriteBundle,
    pub animation: AnimationState,
    pub unit_type: UnitType,
    pub selectable: Selectable,
    pub selectable_type: SelectableType,
    pub state: UnitState,
    pub name: Name,
}


pub fn entity_movement_system(
    mut query: Query<(&mut Unit, &mut Transform, &mut UnitState)>,
    time: Res<Time>,
) {
    for (mut unit, mut transform, mut state) in query.iter_mut() {
        if let Some(target) = state.target_position {
            if unit.can_move_to(target.0, target.1) {
                if unit.move_to(target.0, target.1) {
                    transform.translation = axial_to_world(target.0, target.1);
                    state.target_position = None;
                    state.is_moving = false;
                }
            } else {
                state.target_position = None;
                state.is_moving = false;
            }
        }
    }
}

// Implementation for Unit
impl Unit {
    pub fn new(name: String, sprite: Handle<Image>, q: i32, r: i32) -> Self {
        Self {
            entity: GameEntity {
                name,
                description: "A unit".to_string(),
                entity_type: EntityType::Unit,
                position: (q, r),
                sprite,
            },
            health: DEFAULT_UNIT_HEALTH,
            movement_range: DEFAULT_MOVEMENT_RANGE,
            attack_damage: DEFAULT_ATTACK_DAMAGE,
        }
    }

    #[inline]
    pub fn position(&self) -> (i32, i32) {
        self.entity.position
    }

    #[inline]
    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.health = (self.health - amount).max(0.0);
        self.health <= 0.0  // Returns true if unit is defeated
    }
}

impl Moveable for Unit {
    fn can_move_to(&self, q: i32, r: i32) -> bool {
        let (current_q, current_r) = self.entity.position;
        let distance = calculate_hex_distance(current_q, current_r, q, r);
        distance <= self.movement_range
    }

    fn move_to(&mut self, q: i32, r: i32) -> bool {
        if self.can_move_to(q, r) {
            self.entity.position = (q, r);
            true
        } else {
            false
        }
    }
}

#[inline]
fn calculate_hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs() + (r1 - r2).abs() + (q1 + r1 - q2 - r2).abs()) / 2
}

// Optional: Add a system to handle unit commands
pub fn unit_command_system(
    mut commands: EventReader<UnitCommand>,
    mut query: Query<(&mut UnitState, &Unit)>,
) {
    for command in commands.iter() {
        if let Ok((mut state, unit)) = query.get_mut(command.unit) {
            match command.command_type {
                UnitCommandType::MoveTo(q, r) => {
                    if unit.can_move_to(q, r) {
                        state.target_position = Some((q, r));
                        state.is_moving = true;
                    }
                }
                // Add other command types as needed
            }
        }
    }
}

// Event for unit commands
#[derive(Event)]
pub struct UnitCommand {
    pub unit: Entity,
    pub command_type: UnitCommandType,
}

pub enum UnitCommandType {
    MoveTo(i32, i32),
    // Add other command types as needed
}

pub fn animate_units_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut Handle<Image>, &UnitState)>,
) {
    for (mut anim_state, mut texture, unit_state) in query.iter_mut() {
        let current_animation = match anim_state.current_animation {
            UnitAnimation::Idle => &mut anim_state.idle,
            UnitAnimation::Walking => &mut anim_state.walking,
            UnitAnimation::Attacking => &mut anim_state.attacking,
        };

        if !current_animation.frames.is_empty() {
            current_animation.timer.tick(time.delta());

            if current_animation.timer.just_finished() {
                current_animation.current_frame = 
                    (current_animation.current_frame + 1) % current_animation.frames.len();
                
                // Add debug print to see frame changes
                //println!(
                    //"Changing to frame {} of {}",
                //    current_animation.current_frame,
                //    current_animation.frames.len()
                //);
                
                // Ensure we're within bounds
                if let Some(new_texture) = current_animation.frames.get(current_animation.current_frame) {
                    *texture = new_texture.clone();
                }
            }
        }
    }
}