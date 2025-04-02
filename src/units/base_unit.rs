// in units/base_unit.rs
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::utils::axial_to_world;

#[derive(Component)]
pub struct BaseUnit {
    pub entity: GameEntity,
    pub health: f32,
    pub movement_range: i32,
    pub attack_damage: f32,
}

impl BaseUnit {
    pub fn new(name: String, sprite: Handle<Image>, q: i32, r: i32) -> Self {
        Self {
            entity: GameEntity {
                name,
                description: "A unit".to_string(),
                entity_type: EntityType::Unit,
                position: (q, r),
                sprite,
            },
            health: super::DEFAULT_UNIT_HEALTH as f32,
            movement_range: super::DEFAULT_MOVEMENT_RANGE,
            attack_damage: super::DEFAULT_STRENGTH as f32,
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
