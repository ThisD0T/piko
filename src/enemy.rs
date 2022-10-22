use bevy::prelude::*;

use crate::components::{Enemy, Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_detect);
    }
}

fn enemy_detect(
    mut enemy_query: Query<(&mut Enemy, &Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
        if enemy.spotted_player {
            let distance =
                Vec3::distance(enemy_transform.translation, player_transform.translation);
            if distance < enemy.vision {
                enemy.spotted_player = true;
            }
        }
    }
}

fn enemy_chase(
    mut enemy_query: Query<(&Enemy, &mut Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
}
