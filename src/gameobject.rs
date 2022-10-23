use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    components::{Enemy, EnemyFlock, RunnerEnemy},
};

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_base_enemy(
    mut commands: &mut Commands,
    ascii: &AsciiSheet,
    sprite_index: usize,
    color: Color,
    position: Vec3,
    size: Vec2,
    health: f32,
    vision: f32,
) -> Entity {
    let enemy = spawn_ascii_sprite(&mut commands, &ascii, sprite_index, color, position, size);
    commands
        .entity(enemy)
        .insert(Enemy {
            health,
            vision,
            spotted_player: false,
        })
        .insert(Name::new("Enemy"));

    enemy
}

pub fn spawn_runner_enemy(
    mut commands: &mut Commands,
    ascii: &AsciiSheet,
    sprite_index: usize,
    color: Color,
    position: Vec3,
    size: Vec2,
    health: f32,
    vision: f32,
    speed: f32,
    max_force: f32,
) -> Entity {
    let enemy = spawn_base_enemy(
        &mut commands,
        &ascii,
        sprite_index,
        color,
        position,
        size,
        health,
        vision,
    );
    commands
        .entity(enemy)
        .insert(EnemyFlock {
            speed: speed,
            max_force: max_force,
            velocity: Vec3::splat(0.0),
            acceleration: Vec3::splat(0.0),
            in_view: Vec::new(),
        })
        .insert(RunnerEnemy);

    enemy
}
