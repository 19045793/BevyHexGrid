use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::OrthographicProjection;
use bevy::input::mouse::MouseWheel;
use crate::resources::MouseState;
use crate::constants::input::*;

use bevy::ui::Style;

/// Camera configuration
#[derive(Resource)]
pub struct CameraConfig {
    pub movement_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub x_rotation: f32,
    pub y_rotation: f32,
    z_rotation: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            movement_speed: 500.0,
            zoom_speed: 0.1,
            min_zoom: 0.5,
            max_zoom: 5.0,
            x_rotation: 15.0_f32.to_radians(), // Classic isometric X angle
            y_rotation: -45.0_f32.to_radians(),   // Classic isometric Y angle
            z_rotation: 15.0_f32.to_radians(),   // Classic isometric Y angle
        }
    }
}

/// Component to mark the main game camera
#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1000.0),
                rotation: Quat::from_rotation_x(camera_config.x_rotation)
                    * Quat::from_rotation_y(camera_config.y_rotation)* Quat::from_rotation_z(camera_config.z_rotation),
                scale: Vec3::new(1.0, -1.0, 1.0),
            },
            projection: OrthographicProjection {
                far: 100000.0,     // Extend far plane significantly
                near: -100000.0,   // Extend near plane significantly
                ..default()
            },
            ..default()
        },
        MainCamera,
    ))
    .insert(Name::new("Main Camera"));
}
pub fn camera_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    camera_config: Res<CameraConfig>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut transform_q: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let mut movement = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement.x += 1.0;
    }

    if movement != Vec3::ZERO {
        let (camera, camera_transform) = camera_q.single();
        if let Ok(mut transform) = transform_q.get_single_mut() {
            // Convert movement to world space
            let screen_pos = Vec2::new(0.0, 0.0); // Center of screen
            let screen_pos_moved = screen_pos + Vec2::new(movement.x, movement.y);

            // Get world positions
            if let (Some(world_pos), Some(world_pos_moved)) = (
                camera.viewport_to_world(camera_transform, screen_pos),
                camera.viewport_to_world(camera_transform, screen_pos_moved),
            ) {
                let world_delta = (world_pos_moved.origin - world_pos.origin).normalize() 
                    * camera_config.movement_speed 
                    * time.delta_seconds();
                transform.translation += world_delta;
            }
        }
    }
}

fn get_movement_direction(keyboard_input: &Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    direction
}

pub fn mouse_camera_movement_system(
    buttons: Res<Input<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut transform_q: Query<&mut Transform, With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(position) = window.cursor_position() {
        // Handle press
        if buttons.just_pressed(MouseButton::Left) {
            mouse_state.start_press(position, MouseButton::Left);
        } else if buttons.just_pressed(MouseButton::Right) {
            mouse_state.start_press(position, MouseButton::Right);
        }

        // Handle releases
        if buttons.just_released(MouseButton::Left) || buttons.just_released(MouseButton::Right) {
            mouse_state.end_press();  // Updated to no arguments
        }

        if mouse_state.pressed {
            if mouse_state.check_drag(position) {  // Updated to use check_drag
                if let Ok(mut transform) = transform_q.get_single_mut() {
                    // Convert both current and last position to world coordinates
                    let current_world = if let Some(current_world) = camera.viewport_to_world(
                        camera_transform,
                        position,
                    ) {
                        current_world.origin
                    } else {
                        return;
                    };

                    let last_world = if let Some(last_world) = camera.viewport_to_world(
                        camera_transform,
                        mouse_state.last_position,
                    ) {
                        last_world.origin
                    } else {
                        return;
                    };

                    // Calculate world space delta
                    let world_delta = current_world - last_world;
                    transform.translation -= world_delta;
                }
            }
            
            mouse_state.update_position(position);
        }
    }
}

pub fn camera_zoom_system(
    mut scroll_evr: EventReader<MouseWheel>,
    camera_config: Res<CameraConfig>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if let Ok(mut ortho) = query.get_single_mut() {
        for ev in scroll_evr.iter() {
            let new_scale = (ortho.scale - ev.y * camera_config.zoom_speed)
                .clamp(camera_config.min_zoom, camera_config.max_zoom);
            ortho.scale = new_scale;
        }
    }
}

pub fn camera_angle_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let rotation_speed = 1.0 * time.delta_seconds();
        
        // Adjust X rotation
        if keyboard_input.pressed(KeyCode::I) {
            let new_rotation = Quat::from_rotation_x(-rotation_speed); // Reversed
            transform.rotation = new_rotation * transform.rotation;
        }
        if keyboard_input.pressed(KeyCode::K) {
            let new_rotation = Quat::from_rotation_x(rotation_speed); // Reversed
            transform.rotation = new_rotation * transform.rotation;
        }
        
        // Adjust Y rotation
        if keyboard_input.pressed(KeyCode::J) {
            let new_rotation = Quat::from_rotation_y(rotation_speed); // Reversed
            transform.rotation = new_rotation * transform.rotation;
        }
        if keyboard_input.pressed(KeyCode::L) {
            let new_rotation = Quat::from_rotation_y(-rotation_speed); // Reversed
            transform.rotation = new_rotation * transform.rotation;
        }
    }
}

