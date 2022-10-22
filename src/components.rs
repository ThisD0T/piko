use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: u32,
}

#[derive(Component)]
pub struct CameraFlag;

#[derive(Component)]
pub struct TileCollider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub vision: f32,
    pub spotted_player: bool,
}

#[derive(Component)]
pub struct EnemyFlock {
    pub speed: f32,
}

#[derive(Component)]
pub struct RunnerEnemy;
