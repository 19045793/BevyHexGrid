// in warrior.rs
use bevy::prelude::*;
use bevy::time::Timer;
use crate::components::{AnimationData, AnimationState, UnitAnimation, AnimationSet};
use super::{BaseUnit, UnitBehavior};


#[derive(Component)]
pub struct ElvenArcher {
    pub base: BaseUnit,
    pub special_ability_cooldown: Timer,
}
// Implement the trait for Warrior
impl UnitBehavior for ElvenArcher {
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

impl ElvenArcher {
    pub fn new(base: BaseUnit) -> Self {
        Self {
            base,
            special_ability_cooldown: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }

    pub fn initialize(asset_server: &AssetServer) -> AnimationSet {
        println!("Initializing Elven Archer animations");
        
        let idle_frames = (1..=10).map(|i| {
            let path = format!("textures/units/elvenArcher/idle/frame_{}.png", i);
            println!("Loading idle frame: {}", path);
            asset_server.load(path)
        }).collect();

        let walk_frames = (1..=7).map(|i| {
            let path = format!("textures/units/elvenArcher/walk/frame_{}.png", i);
            println!("Loading walk frame: {}", path);
            asset_server.load(path)
        }).collect();

        let attack_frames = (1..=6).map(|i| {
            let path = format!("textures/units/elvenArcher/attack/frame_{}.png", i);
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