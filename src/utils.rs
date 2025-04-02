use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::constants::{HEX_SIZE, SQRT_3};

/// Coordinate conversion functions
pub mod coordinates {
    use super::*;

    /// Convert axial coordinates to world position
    pub fn axial_to_world(q: i32, r: i32) -> Vec3 {
        let x = HEX_SIZE * SQRT_3 * (q as f32 + r as f32 / 2.0);
        let y = HEX_SIZE * 1.5 * r as f32;
        Vec3::new(x, y, 0.0)
    }

    /// Convert world position to axial coordinates
    pub fn world_to_axial(position: Vec3) -> (i32, i32) {
        let q = (position.x / (HEX_SIZE * SQRT_3) - position.y / (HEX_SIZE * 3.0)).round() as i32;
        let r = (position.y / (HEX_SIZE * 1.5)).round() as i32;
        (q, r)
    }

    /// Calculate distance between two hex coordinates
    pub fn hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
        ((q1 - q2).abs() + (r1 - r2).abs() + (q1 + r1 - q2 - r2).abs()) / 2
    }
}

/// Mesh generation functions
pub mod mesh {
    use super::*;

    /// Generate a hexagon mesh with the given size
    pub fn hexagon_mesh(size: f32) -> Mesh {
        let (vertices, indices) = generate_hex_vertices_and_indices(size);
        create_mesh_from_vertices(vertices, indices, size)
    }

    fn generate_hex_vertices_and_indices(size: f32) -> (Vec<[f32; 3]>, Vec<u32>) {
        let angle = std::f32::consts::PI / 3.0;
        let rotation = std::f32::consts::PI / 6.0;

        // Generate vertex positions
        let positions: Vec<[f32; 3]> = (0..6)
            .map(|i| {
                let theta = angle * i as f32 + rotation;
                [
                    size * theta.cos(),
                    size * theta.sin(),
                    0.0,
                ]
            })
            .collect();

        // Add center vertex
        let mut vertices = vec![[0.0, 0.0, 0.0]];
        vertices.extend_from_slice(&positions);

        // Generate indices
        let indices: Vec<u32> = (1..=6)
            .flat_map(|i| {
                vec![
                    0,
                    i as u32,
                    if i < 6 { (i + 1) as u32 } else { 1 },
                ]
            })
            .collect();

        (vertices, indices)
    }

    fn create_mesh_from_vertices(vertices: Vec<[f32; 3]>, indices: Vec<u32>, size: f32) -> Mesh {
        let normals = vec![[0.0, 0.0, 1.0]; vertices.len()];
        let uvs: Vec<[f32; 2]> = vertices
            .iter()
            .map(|[x, y, _]| {
                let u = (*x / (size * 2.0)) + 0.5;
                let v = (*y / (size * 2.0)) + 0.5;
                [u, v]
            })
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }
}

// Re-export commonly used functions at the module level
pub use coordinates::*;
pub use mesh::*;