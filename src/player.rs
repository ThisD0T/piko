use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        3,
        Color::rgb(0.1, 0.7, 0.4),
        Vec3::new(5.0 * TILE_SIZE, -2.0 * TILE_SIZE, 100.0),
        15.0,
    );
    commands.entity(player).insert(Name::new("Player"));
}
