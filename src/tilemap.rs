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

fn map_gen_manager(

) {

}

pub struct MapBlock {
    x: i32,
    y: i32,
    block_type: String,
    entrance: bool,
    exit: bool,
}

fn gen_map (
    
) {
    let map_size = 5;
    let mut rng = rand::thread_rng();

    let mut map_blocks: Vec<MapBlock> = Vec::new();

    let mut top_open: Vec<MapBlock> = Vec::new();
    let mut bottom_open: Vec<MapBlock> = Vec::new();
    let mut left_open: Vec<MapBlock> = Vec::new();
    let mut right_open: Vec<MapBlock> = Vec::new();

    let options = MatchOptions{
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    // generate a list of map blocks
    for entry in glob_with("./*a*", options) {
        println!("{:?}", path.display());
    };

    // generate a route through the map from the entrance to the exit
    let mut exit_placed: bool = false;
    while exit_placed == false {
        let mut direction_chosen = false;
        while direction_chosen == false {
            let mut direction = rng.gen_range(1..4);
            let mut direction_vector = Vec2::new(0.0, 0.0);


            match direction {
                1 => direction_vector = Vec2::new(1.0, 0.0),
                2 => direction_vector = Vec2::new(-1.0, 0.0),
                3 => direction_vector = Vec2::new(0.0, -1.0),
                4 => direction_vector = Vec2::new(0.0, 1.0),
                _ => println!("there's something weird happening with direction calulation in map_gen")
            }
            direction_chosen = true;
        }
        exit_placed = true;
    }

    // fill in the rest of the map with random tiles
}

fn gen_map_block (
    map_blocks: Vec<MapBlock>
) {

}

