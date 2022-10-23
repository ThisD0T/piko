use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    components::{CameraFlag, Exit, Manager, Player, TileCollider},
    make_new_stage, TILE_SIZE,
};

use crate::tilemap::{MAP_BLOCK_X, MAP_BLOCK_Y};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow)
            .add_system(player_exit)
            .add_system(player_controller);
    }
}

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        3,
        Color::rgb(0.1, 0.7, 0.4),
        Vec3::new(0.0, 0.0, 100.0),
        Vec2::splat(TILE_SIZE * 0.98),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 280.0,
            health: 100,
        });
}

pub struct MoveDirections {
    up: Vec3,
    down: Vec3,
    left: Vec3,
    right: Vec3,
}

fn player_controller(
    mut query: Query<(&Player, &mut Transform), With<Player>>,
    mut tile_query: Query<(&Transform, &TileCollider), (With<TileCollider>, Without<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();

    let move_directions = MoveDirections {
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

    let target_position = Vec3::new(move_vector[0], 0.0, 0.0) + transform.translation;
    if !wall_collision_check(target_position, &tile_query) {
        transform.translation = target_position;
    }

    let target_position = Vec3::new(0.0, move_vector[1], 0.0) + transform.translation;
    if !wall_collision_check(target_position, &tile_query) {
        transform.translation = target_position;
    }
}

fn wall_collision_check(
    target_position: Vec3,
    wall_query: &Query<(&Transform, &TileCollider), (With<TileCollider>, Without<Player>)>,
) -> bool {
    for (wall_transform, wall_collider) in wall_query.iter() {
        let collision = collide(
            target_position,
            Vec2::splat(TILE_SIZE),
            wall_transform.translation,
            wall_collider.size,
        );
        if collision.is_some() {
            return true;
        }
    }
    return false;
}
fn camera_follow(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<CameraFlag>)>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single_mut();

    camera_transform.translation = player_transform.translation;
}

pub fn respawn_player(mut commands: &mut Commands, ascii: &mut Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        3,
        Color::rgb(0.1, 0.7, 0.4),
        Vec3::new(0.0, 0.0, 100.0),
        Vec2::splat(TILE_SIZE * 0.98),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 280.0,
            health: 100,
        });
}

fn player_exit(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    entity_query: Query<Entity, Without<Manager>>,
    player_query: Query<&Transform, With<Player>>,
    exit_query: Query<&Transform, (With<Exit>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let exit_transform = exit_query.single();

    println!(
        "{}",
        Vec3::distance(player_transform.translation, exit_transform.translation)
    );
    if Vec3::distance(player_transform.translation, exit_transform.translation)
        < (110.0 + TILE_SIZE) * 0.98
    {
        println!("making new stage");
        make_new_stage(commands, ascii, entity_query);
    }
}
