use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use glob::{glob_with, MatchOptions};

use rand::prelude::*;

use bevy::prelude::*;

use crate::ascii::spawn_ascii_sprite;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(gen_map);
    }
}

pub fn read_last_map_block(blocks: &mut Vec<MapBlock>) -> Vec2 {
    let last_index = &blocks.len();
    println!("num of map blocks placed: {}", last_index);
    let last_block = &blocks[last_index - 1];

    return Vec2::new(last_block.x, last_block.y);
}

fn map_gen_manager() {}

pub struct MapBlock {
    x: f32,
    y: f32,
    block_id: String,
    entrance: bool,
    exit: bool,
}

fn gen_map() {
    let map_size: f32 = 4.0;
    let map_size_int: i32 = map_size.ceil() as i32;
    let mut rng = rand::thread_rng();

    let mut top_open: Vec<String> = Vec::new();
    let mut bottom_open: Vec<String> = Vec::new();
    let mut left_open: Vec<String> = Vec::new();
    let mut right_open: Vec<String> = Vec::new();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    // generate vectors of map blocks
    for entry in glob_with("../map_blocks/*a*", options)
        .unwrap()
        .filter_map(Result::ok)
    {
        let map_block = entry.display();
        top_open.push(map_block.to_string());
        bottom_open.push(map_block.to_string());
        left_open.push(map_block.to_string());
        right_open.push(map_block.to_string());
    }

    // generate a route through the map from the entrance to the exit

    let mut exit_route: Vec<MapBlock> = Vec::new();
    let start_position: MapBlock = MapBlock {
        x: 0.0,
        y: 0.0,
        block_id: "mb_udlra".to_string(),
        entrance: true,
        exit: false,
    };
    exit_route.push(start_position);

    // find a valid block to add to the route
    let mut exit_route_found = false;
    while exit_route_found == false {
        println!("\n\nlooping again");
        let direction = rng.gen_range(1..4);
        let mut direction_vector = Vec2::new(0.0, 0.0);

        match direction {
            1 => direction_vector = Vec2::new(1.0, 0.0),
            2 => direction_vector = Vec2::new(-1.0, 0.0),
            3 => direction_vector = Vec2::new(0.0, -1.0),
            4 => direction_vector = Vec2::new(0.0, 1.0),
            _ => println!(
                "there's something weird happening with direction calulation in gen_map..."
            ),
        }
        let mut chosen_position = read_last_map_block(&mut exit_route);
        chosen_position = chosen_position + direction_vector;
        println!("position chosen: {}", chosen_position);

        // check to see if the direction chosen to expand the exit route in doesn't step on any
        // previously placed blocks and that it isn't out of bounds
        let mut map_block_chosen_ok: bool = true;

        for placed_block in exit_route.iter() {
            println!(
                "\ncomparison\nplaced block: {}, {}\nrandom position: {}, {}\n",
                placed_block.x, placed_block.y, chosen_position[0], chosen_position[1],
            );
            if placed_block.x.round() == chosen_position[0].round()
                && placed_block.y.round() == chosen_position[1].round()
            {
                map_block_chosen_ok = false;
            }
        }
        // .push the block if the location is not on top of any others
        if map_block_chosen_ok == true {
            let mut map_block = MapBlock {
                x: chosen_position[0],
                y: chosen_position[1],
                block_id: "mb_udlra".to_string(),
                entrance: false,
                exit: false,
            };

            // determine whether the placed map block should be an exit
            // and whether or not the route is finished or not
            if chosen_position[0] >= map_size
                || chosen_position[1] >= map_size
                || chosen_position[0] <= -map_size
                || chosen_position[1] <= -map_size
            {
                let coin_flip = rng.gen_bool(1.0 / 2.0);
                if coin_flip {
                    map_block.exit = true;
                    exit_route.push(map_block);
                    println!("pushed map block, exit route found");
                    println!("\nmap block summary: ");
                    for map_block in exit_route.iter() {
                        println!("{}{}", map_block.x, map_block.y);
                    }
                    break;
                } else {
                    exit_route.push(map_block);
                    println!("pushed map block\n\n");
                }
            } else {
                exit_route.push(map_block);
                println!("pushed map block\n\n");
            }
        }
    }
    // fill in the rest of the map with random tiles
}

fn gen_map_block(map_blocks: Vec<MapBlock>) {}
