use bevy::prelude::*;

use crate::{
    components::{Enemy, EnemyFlock, Node, NodeGraph, Player, TileCollider},
    player::wall_collision_check,
    TILE_SIZE,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_detect)
            .add_system(enemy_chase)
            .add_system(enemy_phys_update)
            .add_system(enemy_hit_detect)
            .add_system(enemy_separation);
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
    let mut steering = desired - enemy.velocity;
    steering = set_magnitude(steering, enemy.max_force);
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
        steering = Vec3::clamp_length_max(steering, enemy.speed * 2.5);
        return steering;
    } else {
        return Vec3::splat(0.0);
    }
}

fn enemy_vision(
    mut enemy_query: Query<(&mut Transform, &Enemy, &mut EnemyFlock), With<Enemy>>,
) -> Vec<Transform> {
    return Vec::new();
}

fn enemy_separation(
    mut enemy_query: Query<(&Transform, &Enemy, &mut EnemyFlock), With<EnemyFlock>>,
) {
    let mut iter = enemy_query.iter_combinations_mut();
    while let Some([(transform, enemy, mut enemy_flock), (transform2, enemy2, enemy_flock2)]) =
        iter.fetch_next()
    {
        let mut something_in_vision = false;
        let mut vision_total = 0;
        let mut steering = Vec3::splat(0.0);
        if Vec3::distance(transform.translation, transform2.translation) < enemy.vision / 2.0 {
            something_in_vision = true;
            let d = Vec3::distance(transform.translation, transform2.translation);
            let mut diff = get_vector(transform.translation, transform2.translation);
            diff = diff / (d * d);
            steering += diff;

            vision_total += 1;
        }

        if something_in_vision {
            // this is to avoid dividing by zero and making things go wonky
            // apply forces
            steering = steering / vision_total as f32;
            steering = set_magnitude(steering, enemy_flock.max_force * 0.5);
            steering = Vec3::clamp_length_max(steering, enemy_flock.speed);
            enemy_flock.acceleration += steering;
        }
    }
}

pub fn set_magnitude(mut vector: Vec3, magnitude: f32) -> Vec3 {
    vector = vector / vector.length();
    vector *= magnitude;
    return vector;
}

fn get_vector(vec_1: Vec3, vec_2: Vec3) -> Vec3 {
    Vec3::new(vec_1[0] - vec_2[0], vec_1[1] - vec_2[1], 0.0)
}

fn enemy_phys_update(
    mut enemy_query: Query<(&mut Transform, &mut EnemyFlock, &Enemy), With<EnemyFlock>>,
    tile_query: Query<
        (&Transform, &TileCollider),
        (With<TileCollider>, Without<Player>, Without<EnemyFlock>),
    >,
    time: Res<Time>,
) {
    for (mut transform, mut enemy_flock, enemy) in enemy_query.iter_mut() {
        if enemy.spotted_player {
            enemy_flock.velocity = enemy_flock.velocity + enemy_flock.acceleration;
            enemy_flock.velocity = Vec3::clamp_length_max(enemy_flock.velocity, enemy_flock.speed);
            enemy_flock.velocity[2] = 0.0;

            let wish_pos = Vec3::new(enemy_flock.velocity[0] * time.delta_seconds(), 0.0, 0.0)
                + transform.translation;
            if !wall_collision_check(wish_pos, &tile_query) {
                transform.translation = wish_pos;
            }

            let wish_pos = Vec3::new(0.0, enemy_flock.velocity[1] * time.delta_seconds(), 0.0)
                + transform.translation;
            if !wall_collision_check(wish_pos, &tile_query) {
                transform.translation = wish_pos;
            }

            enemy_flock.acceleration = Vec3::splat(0.0);
            transform.translation[2] = 0.0;
        }
    }
}

fn enemy_hit_detect(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    for (enemy, enemy_transform) in enemy_query.iter() {
        if Vec3::distance(enemy_transform.translation, player_transform.translation) < TILE_SIZE {
            commands.entity(enemy).despawn();
            player.health -= 1;
        }
    }
}
