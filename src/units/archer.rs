// in warrior.rs
use bevy::prelude::*;
use bevy::time::Timer;
use crate::components::{AnimationData, AnimationState, UnitAnimation, AnimationSet};
use super::{BaseUnit, UnitBehavior};

#[derive(Component)]
pub struct Archer {
    pub base: BaseUnit,
    pub special_ability_cooldown: Timer,
}

// Implement the trait for Warrior
impl UnitBehavior for Archer {
    fn new(base: BaseUnit) -> Self {
        Self {
            base,
            special_ability_cooldown: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }

    fn create_animation_state(animations: &AnimationSet) -> AnimationState {
        AnimationState {
            idle: AnimationData {
                frames: animations.idle.clone().unwrap_or_default(),
                current_frame: 0,
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                is_looping: true,
            },
            walking: AnimationData {
                frames: animations.walk.clone().unwrap_or_default(),
                current_frame: 0,
                timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                is_looping: true,
            },
            attacking: AnimationData {
                frames: animations.attack.clone().unwrap_or_default(),
                current_frame: 0,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                is_looping: false,
            },
            current_animation: UnitAnimation::Idle,
        }
    }

    fn get_scale() -> Vec3 {
        Vec3::new(0.7, -0.7, 1.0)
    }
}

impl Archer {
    pub fn new(base: BaseUnit) -> Self {
        Self {
            base,
            special_ability_cooldown: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }

    pub fn initialize(asset_server: &AssetServer) -> AnimationSet {
        println!("Initializing Archer animations");
        
        let idle_frames = (1..=8).map(|i| {
            let path = format!("textures/units/archer/idle/frame_{}.png", i);
            println!("Loading idle frame: {}", path);
            asset_server.load(path)
        }).collect();

        let walk_frames = (1..=8).map(|i| {
            let path = format!("textures/units/archer/walk/frame_{}.png", i);
            println!("Loading walk frame: {}", path);
            asset_server.load(path)
        }).collect();

        let attack_frames = (1..=28).map(|i| {
            let path = format!("textures/units/archer/attack/frame_{}.png", i);
            println!("Loading attack frame: {}", path);
            asset_server.load(path)
        }).collect();

        AnimationSet {
            idle: Some(idle_frames),
            walk: Some(walk_frames),
            attack: Some(attack_frames),
        }
    }
    pub fn special_ability(&mut self) {
        // Warrior-specific special ability
    }
}