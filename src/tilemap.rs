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
    components::TileCollider,
    gameobject::spawn_runner_enemy,
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

fn random_map_id(id_list: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    return id_list.choose(&mut rng).unwrap().to_string();
}

// simplified map generation system because the last one was ridiculous
fn generate_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut rng = thread_rng();

    let map_size = 2;
    let mut map_blocks: Vec<MapBlock> = Vec::new();
    let mut potential_exits: Vec<MapBlock> = Vec::new();
    let mut map_block_ids: Vec<String> = Vec::new();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with("map_blocks/*m*", options).unwrap() {
        match entry {
            Ok(path) => map_block_ids.push(path.display().to_string()),
            Err(e) => println!("{}", e),
        }
    }

    for id in map_block_ids.iter() {
        println!("{}", id);
    }

    // doing this in columns not rows
    for x in -map_size..map_size + 1 {
        for y in -map_size..map_size + 1 {
            let map_id = random_map_id(&map_block_ids);

            let mut map_block = MapBlock {
                x: x - map_size / 2,
                y: y - map_size / 2,
                block_id: map_id,
                // blocks randomly
                entrance: false,
                exit: false,
                tiles: Vec::new(),
            };

            if x == 0 && y == 0 {
                map_block.entrance = true;
            }

            if map_block.x == map_size
                || map_block.x == -map_size
                || map_block.y == map_size
                || map_block.y == -map_size
            {
                potential_exits.push(map_block);
            } else {
                map_blocks.push(map_block);
            }
        }
    }
    // grab a random exit and then concatenate both of the vectors of map blocks
    potential_exits.choose_mut(&mut rng).unwrap().block_id = "map_blocks/exit.txt".to_string();
    map_blocks.append(&mut potential_exits);

    for mut map_block in map_blocks.iter_mut() {
        if map_block.entrance {
            map_block.block_id = "map_blocks/entrance.txt".to_string();
        }
    }

    draw_map_blocks(commands, ascii, map_blocks, map_size);
}

fn draw_map_blocks(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    map_blocks: Vec<MapBlock>,
    map_size: i32,
) {
    let mut rng = rand::thread_rng();

    let mut tiles = Vec::new();

    let mut enemies: Vec<Entity> = Vec::new();

    let total_map_size = ((((map_size as f32) * 2.0) + 1.0) * MAP_BLOCK_X) * TILE_SIZE;

    let half_block_size = (32 / 2) * TILE_SIZE as i32;

    for map_block in map_blocks {
        let map_file = File::open(map_block.block_id).expect("Map file not found");
        // iterate through all the characters in the map block file
        for (y, line) in BufReader::new(map_file).lines().enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    // spawn the walls and entities in the map block according the the text file
                    // that defines it

                    if char == '#' {
                        let tile_translation = Vec3::new(
                            (MAP_BLOCK_X * 0.98 * map_block.x as f32 + x as f32 * TILE_SIZE * 0.98)
                                + half_block_size as f32,
                            (MAP_BLOCK_Y * 0.98 * map_block.y as f32
                                + (y as f32) * TILE_SIZE * 0.98)
                                + half_block_size as f32,
                            1.0,
                        );

                        let tile = spawn_ascii_sprite(
                            &mut commands,
                            &ascii,
                            0,
                            Color::rgb_u8(255, 255, 255),
                            tile_translation,
                            Vec2::splat(TILE_SIZE),
                        );

                        commands.entity(tile).insert(TileCollider {
                            size: Vec2::splat(TILE_SIZE),
                        });

                        tiles.push(tile);
                    } else if char == 'E' {
                        let tile_translation = Vec3::new(
                            (MAP_BLOCK_X * 0.98 * map_block.x as f32 + x as f32 * TILE_SIZE * 0.98)
                                + half_block_size as f32,
                            (MAP_BLOCK_Y * 0.98 * map_block.y as f32
                                + (y as f32) * TILE_SIZE * 0.98)
                                + half_block_size as f32,
                            1.0,
                        );

                        let tile = spawn_ascii_sprite(
                            &mut commands,
                            &ascii,
                            char as usize,
                            Color::rgb_u8(255, 255, 255),
                            tile_translation,
                            Vec2::splat(TILE_SIZE),
                        );

                        commands.entity(tile).insert(TileCollider {
                            size: Vec2::splat(TILE_SIZE),
                        });

                        tiles.push(tile);
                    } else if char == '7' {
                        if rng.gen_bool(1.0 / 2.0) {
                            let tile_translation = Vec3::new(
                                (MAP_BLOCK_X * 0.98 * map_block.x as f32
                                    + x as f32 * TILE_SIZE * 0.98)
                                    + half_block_size as f32,
                                (MAP_BLOCK_Y * 0.98 * map_block.y as f32
                                    + (y as f32) * TILE_SIZE * 0.98)
                                    + half_block_size as f32,
                                1.0,
                            );

                            let enemy_7 = spawn_runner_enemy(
                                &mut commands,
                                &ascii,
                                4,
                                Color::rgb_u8(255, 255, 255),
                                tile_translation,
                                Vec2::splat(TILE_SIZE),
                                1.0,
                                TILE_SIZE * 32.0,
                                15.0,
                            );
                        }
                    } else {
                    }
                }
            }
        }
    }
    let map_border_size: f32 = (((map_size as f32) * 2.0) + 1.0) * MAP_BLOCK_X;
    let map_border_size_in_tiles: i32 = map_border_size as i32 / TILE_SIZE as i32;

    let map_block_adjust = MAP_BLOCK_X * 0.02;

    // top border
    let mut tile_translation = Vec3::new(0.0, map_border_size / 2.0 * 0.98, 0.0);
    let mut tile_size = Vec2::new(map_border_size * 0.98, TILE_SIZE);
    let top_border = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb_u8(255, 255, 255),
        tile_translation,
        tile_size,
    );
    commands
        .entity(top_border)
        .insert(TileCollider { size: tile_size })
        .insert(Name::new("Top Border"));

    // bottom border
    tile_translation[1] = -tile_translation[1];
    let bottom_border = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb_u8(255, 255, 255),
        tile_translation,
        tile_size,
    );
    commands
        .entity(bottom_border)
        .insert(TileCollider { size: tile_size })
        .insert(Name::new("Bottom Border"));

    // left border
    tile_translation = Vec3::new(-map_border_size / 2.0 * 0.98, 0.0, 0.0);
    tile_size = Vec2::new(TILE_SIZE, map_border_size * 0.98);
    let left_border = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb_u8(255, 255, 255),
        tile_translation,
        tile_size,
    );
    commands
        .entity(left_border)
        .insert(TileCollider { size: tile_size })
        .insert(Name::new("Left Border"));

    // right border
    tile_translation = Vec3::new(map_border_size / 2.0 * 0.98, 0.0, 0.0);
    let right_border = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb_u8(255, 255, 255),
        tile_translation,
        tile_size,
    );
    commands
        .entity(right_border)
        .insert(TileCollider { size: tile_size })
        .insert(Name::new("Right Border"));

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
