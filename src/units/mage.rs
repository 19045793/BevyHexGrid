use bevy::prelude::*;
use bevy::time::Timer;
use super::BaseUnit;

// In components/warrior.rs
#[derive(Component)]
pub struct Mage {
    pub base: BaseUnit,
    pub special_ability_cooldown: Timer,
}

impl Mage {
    pub fn new(base: BaseUnit) -> Self {
        Self {
            base,
            special_ability_cooldown: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }

    pub fn special_ability(&mut self) {
        // Warrior-specific special ability
    }
}