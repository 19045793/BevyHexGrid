use crate::components::SelectableType;

/// Hexagon grid constants
pub mod grid {
    /// Size of each hexagon
    pub const HEX_SIZE: f32 = 30.0;
    /// Radius of the grid in hexes
    pub const GRID_RADIUS: i32 = 58;
    /// Square root of 3 (precalculated for efficiency)
    pub const SQRT_3: f32 = 1.7320508;
}



/// Input constants
pub mod input {
    /// Threshold for detecting drag motion
    pub const DRAG_THRESHOLD: f32 = 1.0;
    /// Camera movement speed
    pub const CAMERA_SPEED: f32 = 500.0;
}


pub const TERRAIN_Z: f32 = 0.0;
pub const UNIT_Z: f32 = 1.0;

pub const HOVER_THRESHOLD_SQUARED: f32 = 25.0; // 5.0 squared
pub const DRAG_THRESHOLD_SQUARED: f32 = 25.0; // 5.0 squared


pub mod selection {
    use crate::systems::MainCamera;

    use super::*;
    use bevy::prelude::*;
    
    #[derive(Clone, Copy)]
    pub struct SelectionThreshold {
        pub threshold: f32,
        pub priority: i32,
    }

    pub mod selection {
        use super::*;
        
        #[derive(Clone, Copy)]
        pub struct SelectionThreshold {
            pub threshold: f32,
            pub priority: i32,
        }
    
        pub fn get_threshold(
            selectable_type: &SelectableType,
            projection: &OrthographicProjection,
            transform: &Transform,
        ) -> SelectionThreshold {
            let base_threshold = match selectable_type {
                SelectableType::Tile => SelectionThreshold { 
                    threshold: super::HEX_SIZE * 1.5,
                    priority: 0 
                },
                SelectableType::Unit => SelectionThreshold { 
                    threshold: super::HEX_SIZE * 0.4,
                    priority: 2 
                },
                SelectableType::Building => SelectionThreshold { 
                    threshold: super::HEX_SIZE * 0.4,
                    priority: 2 
                },
                SelectableType::UI => SelectionThreshold { 
                    threshold: super::HEX_SIZE * 0.2,
                    priority: 1 
                },
            };
    
            // Get camera's rotation angles
            let (x_rot, y_rot, z_rot) = transform.rotation.to_euler(EulerRot::XYZ);
            
            // Calculate scale factor based on camera rotation
            let rotation_scale = (x_rot.cos() * y_rot.cos() * z_rot.cos()).abs();
            
            SelectionThreshold {
                // Adjust threshold based on both zoom and rotation
                threshold: base_threshold.threshold * projection.scale * (1.0 / rotation_scale),
                priority: base_threshold.priority,
            }
        }
    }
}

// Re-export commonly used constants at the module level
pub use grid::*;
pub use input::*;