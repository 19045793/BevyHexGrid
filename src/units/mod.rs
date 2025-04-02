// in units/mod.rs
use bevy::prelude::*;
use crate::components::{AnimationSet, AnimationState};

pub trait UnitBehavior: Component {
    fn new(base: BaseUnit) -> Self;
    fn create_animation_state(animations: &AnimationSet) -> AnimationState;
    fn get_scale() -> Vec3;
}


// in src/units/mod.rs
mod base_unit;
mod warrior;
mod archer;
mod mage;
mod elvenArcher;

pub use base_unit::*;
pub use warrior::*;
pub use archer::*;
pub use mage::*;
pub use elvenArcher::*;

// Shared unit constants
pub const DEFAULT_UNIT_HEALTH: i32 = 100;
pub const DEFAULT_MOVEMENT_RANGE: i32 = 1;
pub const DEFAULT_STRENGTH: i32 = 100;