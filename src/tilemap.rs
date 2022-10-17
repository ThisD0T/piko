use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use glob::{glob_with, MatchOptions};

use rand::prelude::*;

use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
};

pub const MAP_BLOCK_X: f32 = 32.0 * TILE_SIZE;
pub const MAP_BLOCK_Y: f32 = 32.0 * TILE_SIZE;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_map);
    }
}

fn map_gen_manager() {}

pub struct MapBlock {
    x: i32,
    y: i32,
    block_id: String,
    entrance: bool,
    exit: bool,
    tiles: Vec<Entity>,
}

// simplified map generation system because the last one was ridiculous
fn generate_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let map_size = 3;

    let mut map_blocks: Vec<MapBlock> = Vec::new();

    let mut map_block_ids: Vec<String> = Vec::new();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with("map_blocks/*m*", options).unwrap() {
        match entry {
            Ok(path) => map_block_ids.push(path.display().to_string()),
            Err(e) => println!("{}", e)
        }
    }

    // doing this in columns not rows
    for x in -map_size..map_size + 1 {
        for y in -map_size..map_size {
            let map_block = MapBlock {
                x: x - map_size / 2,
                y: y - map_size / 2,
                block_id: "/home/thisdot/dev/rust/piko/map_blocks/mb_udlra.txt".to_string(),
                // blocks randomly
                entrance: false,
                exit: false,
                tiles: Vec::new(),
            };
            map_blocks.push(map_block);
        }
    }

    draw_map_blocks(commands, ascii, map_blocks, map_size);
}

fn draw_map_blocks(mut commands: Commands, ascii: Res<AsciiSheet>, map_blocks: Vec<MapBlock>, map_size: i32) {
    let mut tiles = Vec::new();

    let mut enemies = Vec::new();

    let total_map_size = ((((map_size as f32) * 2.0) + 1.0) * MAP_BLOCK_X) * TILE_SIZE;

    let half_block_size = (32 / 2) * TILE_SIZE as i32;
    println!("{}", half_block_size);

    for map_block in map_blocks {
        let map_file = File::open(map_block.block_id).expect("Map file not found");
        // iterate through all the characters in the map block file
        for (y, line) in BufReader::new(map_file).lines().enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    // spawn the walls and entities in the map block according the the text file
                    // that defines it

                    if char == '.' {
                    } else if char == '7' {
                        let enemy_7 = commands.spawn().id();
                        enemies.push(enemy_7);
                    } else {
                        let tile_translation = Vec3::new(
                            (MAP_BLOCK_X * map_block.x as f32 + x as f32 * TILE_SIZE) + half_block_size as f32,
                            (MAP_BLOCK_Y * map_block.y as f32 + (y as f32) * TILE_SIZE) + half_block_size as f32,
                            1.0,
                        );

                        let tile = spawn_ascii_sprite(
                            &mut commands,
                            &ascii,
                            char as usize,
                            Color::rgb_u8(255, 255, 255),
                            tile_translation,
                            TILE_SIZE,
                        );

                        tiles.push(tile);
                    }
                }
            }
        }

        let map_border_size_in_tiles: i32 = ((((map_size as f32) * 2.0) + 1.0) / TILE_SIZE).round() as i32;
        let map_border_size_f32: f32 = map_border_size_in_tiles as f32 * TILE_SIZE;

        for i in 0..map_border_size_in_tiles {
            println!("spawning border tile");
            let tile_translation = Vec3::new((i as f32 * TILE_SIZE) - map_border_size_f32 / 2.0, map_border_size_f32 / 2.0, 0.0);
            let border_tile = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                4,
                Color::rgb_u8(255, 255, 255),
                tile_translation,
                TILE_SIZE,
                );

            commands.entity(border_tile).insert(Name::new("Border Tile"));

            tiles.push(border_tile);
        }
        

    }
    let mut map = commands.spawn_bundle(SpatialBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        global_transform: GlobalTransform::default(),
        ..default()
    });

    map.insert(Name::new("Map"));

    map.push_children(&tiles);
}
