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
pub const MAP_BLOCK_Y: f32 = 16.0 * TILE_SIZE;

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
    for entry in glob_with("../map_blocks/m*b", options).unwrap() {
        if let Ok(path) = entry {
            map_block_ids.push(path.display().to_string());
            println!("{:?}", path.display().to_string());
        }
    }

    // doing this in columns not rows
    for x in -map_size..map_size {
        for y in -map_size..map_size {
            let map_block = MapBlock {
                x: x - map_size,
                y: y - map_size,
                block_id: "mb_udlra.txt".to_string(),
                // blocks randomly
                entrance: false,
                exit: false,
                tiles: Vec::new(),
            };
            map_blocks.push(map_block);
        }
    }

    draw_map_blocks(commands, ascii, map_blocks);
}

fn draw_map_blocks(mut commands: Commands, ascii: Res<AsciiSheet>, map_blocks: Vec<MapBlock>) {
    for mut map_block in map_blocks {
        let map_file = File::open("/home/thisdot/dev/rust/piko/map_blocks/mb_udlra.txt")
            .expect("Map file not found");

        // iterate through all the characters in the map block file
        for (y, line) in BufReader::new(map_file).lines().enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    if char == '.' {
                    } else {
                        let tile_translation = Vec3::new(
                            MAP_BLOCK_X * map_block.x as f32 + x as f32 * TILE_SIZE,
                            MAP_BLOCK_Y * map_block.y as f32 + (y as f32) * TILE_SIZE,
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

                        map_block.tiles.push(tile);
                    }
                }
            }
        }
    }
}
