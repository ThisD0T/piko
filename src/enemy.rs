use bevy::prelude::*;

use crate::{
    components::{Enemy, EnemyFlock, Node, NodeGraph, Player, TileCollider},
    TILE_SIZE,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_detect)
            .add_system(enemy_chase)
            .add_system(enemy_phys_update);
    }
}

fn enemy_detect(
    mut enemy_query: Query<(&mut Enemy, &Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
        let distance = Vec3::distance(enemy_transform.translation, player_transform.translation);
        if distance < enemy.vision {
            enemy.spotted_player = true;
        }
    }
}

fn enemy_chase(
    mut enemy_query: Query<(&Enemy, &mut EnemyFlock, &mut Transform), With<Enemy>>,
    player_query: Query<&Transform, (Without<Enemy>, With<Player>)>,
    tile_query: Query<&Transform, (Without<Player>, Without<Enemy>, With<TileCollider>)>,
) {
    let player_transform = player_query.single();

    for (enemy, mut enemy_flock, enemy_transform) in enemy_query.iter_mut() {
        if enemy.spotted_player {
            let main_impulse = enemy_seek(
                &mut enemy_flock,
                enemy_transform.translation,
                player_transform.translation,
            );

            let mut near_tiles: Vec<&Transform> = Vec::new();
            for tile_transform in tile_query.iter() {
                if Vec3::distance(enemy_transform.translation, tile_transform.translation)
                    < TILE_SIZE * 2.0
                {
                    near_tiles.push(&tile_transform);
                }
            }

            if near_tiles.len() > 0 {
                let tile_avoid_steering: Vec3 =
                    enemy_flee(&mut enemy_flock, enemy_transform.translation, near_tiles);
                enemy_flock.acceleration += tile_avoid_steering;
            }
            enemy_flock.acceleration += main_impulse;
        }
    }
}

fn enemy_seek(enemy: &mut EnemyFlock, enemy_translation: Vec3, target: Vec3) -> Vec3 {
    let mut desired = target - enemy_translation;
    // let mut steering = desired - enemy.velocity;
    // steering = set_magnitude(steering, enemy.max_force);
    desired = set_magnitude(desired, enemy.max_force);
    return desired;
}

fn enemy_flee(
    enemy: &mut EnemyFlock,
    enemy_translation: Vec3,
    avoid_list: Vec<&Transform>,
) -> Vec3 {
    let mut steering = Vec3::splat(0.0);
    if avoid_list.len() > 0 {
        for avoid in avoid_list.iter() {
            let d = Vec3::distance(avoid.translation, enemy_translation);
            let mut diff = get_vector(enemy_translation, avoid.translation);
            diff = diff / (d * d);
            steering += diff;
        }
        steering = steering / (avoid_list.len() as f32).round();
        steering = set_magnitude(steering, enemy.max_force);
        // steering = steering - enemy.velocity; // steering formula
        steering = Vec3::clamp_length_max(steering, enemy.speed * 1.5);
        return steering;
    } else {
        return Vec3::splat(0.0);
    }
}
/*
fn enemy_vision(mut enemy_query: Query<(&mut Transform, &Enemy, &mut EnemyFlock), With<Enemy>>) {
    let mut transform_list: Vec<&mut Transform> = Vec::new();
    for i in enemy_query.len() {}
}

fn enemy_separation(enemy_query: Query<(&Transform, &Enemy, &mut EnemyFlock), With<EnemyFlock>>) {
    for (transform, enemy, mut enemy_flock) in enemy_query.iter_mut() {}
}
*/
fn set_magnitude(mut vector: Vec3, magnitude: f32) -> Vec3 {
    vector = vector / vector.length();
    vector *= magnitude;
    return vector;
}

fn get_vector(vec_1: Vec3, vec_2: Vec3) -> Vec3 {
    return Vec3::new(vec_1[0] - vec_2[0], vec_1[1] - vec_2[1], 0.0);
}

fn enemy_phys_update(
    mut enemy_query: Query<(&mut Transform, &mut EnemyFlock, &Enemy), With<EnemyFlock>>,
) {
    for (mut transform, mut enemy_flock, enemy) in enemy_query.iter_mut() {
        if enemy.spotted_player {
            enemy_flock.velocity = enemy_flock.velocity + enemy_flock.acceleration;
            enemy_flock.velocity = Vec3::clamp_length_max(enemy_flock.velocity, enemy_flock.speed);

            transform.translation += enemy_flock.velocity;

            enemy_flock.acceleration = Vec3::splat(0.0);
            transform.translation[2] = 0.0;
        }
    }
}
