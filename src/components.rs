use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: u32,
    pub shoot_timer: Timer,
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
    pub max_force: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub in_view: Vec<Transform>,
}

#[derive(Component)]
pub struct RunnerEnemy;

pub struct Node {
    pub x: i32,
    pub y: i32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub previous: Vec<i32>,
}

pub struct NodeGraph(Vec<Vec<Node>>);

#[derive(Component)]
pub struct Manager;

#[derive(Component)]
pub struct Exit;

#[derive(Component)]
pub struct Bullet {
    pub move_vector: Vec3,
}
