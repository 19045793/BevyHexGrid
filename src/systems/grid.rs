use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::utils::HashMap;
use rand::{prelude::*, thread_rng};
use bevy::ui::Style;
use crate::components::*;
use crate::resources::TerrainTextureSet;
use crate::utils::*;
use crate::constants::{GRID_RADIUS, HEX_SIZE};
use crate::constants::SQRT_3;
use noise::{
    MultiFractal, NoiseFn, Fbm, Turbulence, Perlin,
    utils::{NoiseMapBuilder, PlaneMapBuilder},
};

// System to initialize the texture resource
pub fn setup_terrain_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("Starting setup_terrain_textures");
    
    let terrains = load_terrains(&asset_server);
    //println!("Loaded {} terrains", terrains.len());
    
    let mut texture_variants = HashMap::new();
    
    for terrain in &terrains {
        //println!("Processing terrain: {} with {} texture variants", 
        //    terrain.name, 
        //    terrain.texture_handles.len()
        //);
        texture_variants.insert(terrain.name.clone(), terrain.texture_handles.clone());
    }
    
    commands.insert_resource(TerrainTextureSet {
        terrains,
        texture_variants,
    });
    
    println!("Finished setup_terrain_textures");
}

pub fn draw_hex_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Starting draw_hex_grid and terrain setup");
    
    // First set up the terrain textures
    let terrains = load_terrains(&asset_server);
    println!("Loaded {} terrains", terrains.len());
    
    let mut texture_variants = HashMap::new();
    
    for terrain in &terrains {
        texture_variants.insert(terrain.name.clone(), terrain.texture_handles.clone());
    }
    
    let terrain_set = TerrainTextureSet {
        terrains: terrains.clone(),
        texture_variants: texture_variants.clone(),
    };
    
    commands.insert_resource(terrain_set);
    
    // Now generate and draw the hex grid
    let tiles = generate_hex_grid(GRID_RADIUS, &terrains);

    // Calculate hex dimensions
    let width = HEX_SIZE * SQRT_3;
    let height = HEX_SIZE * 2.0;

    for tile in tiles {
        if let Some(terrain_textures) = texture_variants.get(&tile.terrain) {
            if let Some(texture) = terrain_textures.get(tile.texture_variant) {
                // Create the tile entity
                commands.spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(width, height)),
                            color: Color::WHITE,
                            anchor: Anchor::Center,
                            ..default()
                        },
                        transform: Transform::from_translation(tile.position)
                            .with_rotation(Quat::from_rotation_z(0.0))
                            .with_scale(Vec3::new(1.0, -1.0, 1.0)),
                        ..default()
                    },
                    tile.clone(),  // The Tile component itself implements Selectable
                    Selectable::default(),
                    HexSprite {    // Add HexSprite component if you need it
                        orientation: 0.0,
                    },
                ));
                
            } else {
                println!("Failed to get texture variant {} for terrain {}", 
                    tile.texture_variant, tile.terrain);
            }
        } else {
            println!("Failed to get textures for terrain: {}", tile.terrain);
        }
    }
    
    println!("Finished draw_hex_grid");
}

fn load_terrains(asset_server: &Res<AssetServer>) -> Vec<Terrain> {
    let terrain_defs = vec![
        (
            "sand",
            vec![
                "textures/sand_07.png",
                "textures/sand_12.png",
                "textures/sand_13.png",
                "textures/sand_15.png",
            ],
        ),
        (
            "cactus",
            vec![
                "textures/sand_14.png",
            ],
        ),
        (
            "alien",
            vec![
                "textures/mars_07.png",
                "textures/mars_12.png",
            ],
        ),
        (
            "alienForest",
            vec![
                "textures/mars_13.png",
            ],
        ),

        (
            "dirt",
            vec![
                "textures/dirt_06.png",
                "textures/dirt_12.png",
            ],
        ),
        (
            "bigMountain",
            vec![
                "textures/dirt_18.png",
            ],
        ),
        (
            "dirtRocks",
            vec![
                "textures/dirt_15.png",
                "textures/dirt_16.png",
            ],
        ),
        (
            "greenMountain",
            vec![
                "textures/grass_14.png",
            ],
        ),
        (
            "greenRocks",
            vec![
                "textures/grass_15.png",
            ],
        ),
        (
            "sandRocks",
            vec![
                "textures/sand_16.png",
                "textures/sand_17.png",
                "textures/sand_18.png",
            ],
        ),
        (
            "grass",
            vec![
                "textures/grass_05.png",
                "textures/grass_10.png",
                "textures/grass_11.png",
            ],
        ),
        (
            "grassForest",
            vec![
                "textures/grass_12.png",
                "textures/grass_13.png",
            ],
        ),
        (
            "alienRocks",
            vec![
                "textures/mars_17.png",  // You'll need to adjust these texture paths
                "textures/mars_18.png",  // You'll need to adjust these texture paths
                "textures/mars_19.png",  // to match your actual alien rock textures
            ],
        ),
        (
            "alienMountain",
            vec![
                "textures/mars_15.png",  // You'll need to adjust this texture path
            ],
        ),
    ];

    terrain_defs
        .iter()
        .map(|(name, paths)| Terrain {
            name: name.to_string(),
            texture_handles: paths.iter().map(|path| asset_server.load(*path)).collect(),
        })
        .collect()
}


fn generate_hex_grid(radius: i32, terrains: &[Terrain]) -> Vec<Tile> {
    println!("Starting generate_hex_grid with radius {} and {} terrains", radius, terrains.len());
    
    let mut tiles = Vec::new();
    let mut rng = thread_rng();
    let mut id_counter = 0;

    // Random offsets to avoid sampling near origin
    let offset_x = rng.gen_range(-1000.0..1000.0);
    let offset_z = rng.gen_range(-1000.0..1000.0);

     // Create multi-octave noise generators
    let biome_noise = Fbm::<Perlin>::new(rng.gen())
        .set_octaves(6)        
        .set_frequency(0.02)   
        .set_persistence(0.6); 

    let mountain_noise = Fbm::<Perlin>::new(rng.gen())
        .set_octaves(4)        
        .set_frequency(0.025)  
        .set_persistence(0.5);

    let feature_noise = Turbulence::<Perlin, Perlin>::new(Perlin::new(rng.gen()))
        .set_frequency(0.03)   
        .set_power(0.7);       

 // ... later in the function, modify these sections:
    // Helper function to convert axial coordinates to noise space
    fn get_noise_coords(q: i32, r: i32, offset_x: f64, offset_z: f64) -> [f64; 2] {
        // Increased the scaling factors slightly for more variation
        let x = (q as f64) * 1.0 - (r as f64) * 0.6 + offset_x;
        let z = (r as f64) * 0.9 + offset_z;
        [x, z]
    }

    // First pass: Generate all tiles
    let mut initial_tiles = HashMap::new();
    
    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);
        
        for r in r1..=r2 {
            let noise_pos = get_noise_coords(q, r, offset_x, offset_z);
            
            // Debug print to check coordinate distribution
            // println!("q: {}, r: {}, noise_x: {}, noise_z: {}", 
            //     q, r, noise_pos[0], noise_pos[1]);
            
            // Get base biome
            let biome_value = biome_noise.get(noise_pos);
            
             // Determine base biome (adjusted thresholds for desired frequencies)
             let base_biome = if biome_value > 0.0 {  // ~50% grass
                "grass"
            } else if biome_value > -0.4 {            // ~20% dirt
                "dirt"
            } else if biome_value > -0.75 {            // ~17.5% sand
                "sand"
            } else {                                  // ~12.5% alien
                "alien"
            };

            // Check for mountains (independent of biome)
            let mountain_value = mountain_noise.get(noise_pos);
            let feature_value = feature_noise.get(noise_pos);
            
            let terrain_type = if mountain_value > 0.87 {  // Made mountains rarer
                // Mountains override base biome
                match base_biome {
                    "grass" => "greenMountain",
                    "alien" => "alienMountain",  // Added alien mountain
                    _ => "bigMountain",
                }
            } else if feature_value > 0.85 {  // Made cactus very rare
                // Very rare features
                match base_biome {
                    "sand" => "cactus",
                    _ => base_biome, // Fallback to base biome
                }
            } else if feature_value > 0.3 {  // Made rocks more common
                // Rock formations
                match base_biome {
                    "grass" => "greenRocks",
                    "dirt" => "dirtRocks",
                    "sand" => "sandRocks",
                    "alien" => "alienRocks",  // Added alien rocks
                    _ => base_biome,
                }
            } else if feature_value > -0.1 {  // Made forests much more common
                // Forests
                match base_biome {
                    "grass" => "grassForest",
                    "alien" => "alienForest",
                    _ => base_biome,
                }
            } else {
                base_biome
            };
            initial_tiles.insert((q, r), terrain_type.to_string());
        }
    }

    // Second pass: Remove isolated tiles
    for q in -radius..=radius {
        let r1 = (-radius).max(-q - radius);
        let r2 = radius.min(-q + radius);
        
        for r in r1..=r2 {
            if let Some(terrain_type) = initial_tiles.get(&(q, r)).cloned() {
                let position = axial_to_world(q, r);
                
                // Check neighbors
                let neighbors = get_hex_neighbors(q, r);
                let different_neighbors = neighbors.iter()
                    .filter(|(nq, nr)| {
                        initial_tiles.get(&(*nq, *nr))
                            .map_or(false, |t| *t != terrain_type)
                    })
                    .count();

                // If isolated (5+ different neighbors), convert to most common neighbor type
                let final_terrain = if different_neighbors >= 5 {
                    let most_common_neighbor = neighbors.iter()
                        .filter_map(|pos| initial_tiles.get(pos))
                        .fold(HashMap::new(), |mut acc, t| {
                            *acc.entry(t).or_insert(0) += 1;
                            acc
                        })
                        .into_iter()
                        .max_by_key(|&(_, count)| count)
                        .map(|(terrain, _)| terrain.clone())
                        .unwrap_or(terrain_type);
                    most_common_neighbor
                } else {
                    terrain_type
                };

                // Find terrain definition
                let terrain_def = terrains.iter()
                    .find(|t| t.name == final_terrain)
                    .unwrap();

                // Use different noise coordinates for variant selection
                let variant_coords = get_noise_coords(q * 2, r * 2, offset_x * 2.0, offset_z * 2.0);
                let variant_value = feature_noise.get(variant_coords);
                let texture_variant = (variant_value.abs() * terrain_def.texture_handles.len() as f64) as usize
                    % terrain_def.texture_handles.len();

                tiles.push(Tile {
                    id: id_counter,
                    q,
                    r,
                    position,
                    terrain: final_terrain,
                    texture_variant,
                });
                
                id_counter += 1;
            }
        }
    }
    
    println!("Finished generate_hex_grid with {} tiles", tiles.len());
    tiles
}

fn get_hex_neighbors(q: i32, r: i32) -> Vec<(i32, i32)> {
    vec![
        (q+1, r), (q+1, r-1), (q, r-1),
        (q-1, r), (q-1, r+1), (q, r+1)
    ]
}

pub fn hex_sprite_system(
    mut sprites: Query<(&mut Transform, &HexSprite)>,
) {
    for (mut transform, hex_sprite) in sprites.iter_mut() {
        // Add any hex-specific sprite behavior here
        // For example, smooth rotation transitions, etc.
    }
}