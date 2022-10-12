use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    components::{Player, CameraFlag},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_controller);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        3,
        Color::rgb(0.1, 0.7, 0.4),
        Vec3::new(0.0, 0.0, 100.0),
        15.0,
    );
    commands.entity(player)
        .insert(Name::new("Player"))
        .insert(Player{speed: 80.0, health: 100});
}

pub struct MoveDirections {
    up: Vec3,
    down: Vec3,
    left: Vec3,
    right: Vec3,
}

fn player_controller(
    mut query: Query<(&Player, &mut Transform), With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();

    let move_directions= MoveDirections {
        up: Vec3::new(0.0, player.speed * time.delta_seconds(), 0.0),
        down: Vec3::new(0.0, player.speed * time.delta_seconds() * -1.0, 0.0),
        left: Vec3::new((player.speed * time.delta_seconds()) * -1.0, 0.0, 0.0),
        right: Vec3::new(player.speed * time.delta_seconds(), 0.0, 0.0),
        };

    let mut move_vector = Vec3::splat(0.0);

    if keys.pressed(KeyCode::W) {
        move_vector += move_directions.up;
    }
    if keys.pressed(KeyCode::S) {
        move_vector += move_directions.down;
    }

    if keys.pressed(KeyCode::A) {
        move_vector += move_directions.left;
    } 
    if keys.pressed(KeyCode::D) {
        move_vector += move_directions.right;
    }

    move_vector = Vec3::clamp_length(move_vector, 0.0, player.speed);
    transform.translation += move_vector;

}

fn camera_follow(
    mut camera_query: Query<&mut Transform, With<CameraFlag>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single_mut();;

    camera_transform.translation = player_transform.translation;
}

