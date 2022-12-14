use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuFlag;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: i32,
    pub shoot_timer: Timer,
    pub ammo: i32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub max_speed: f32,
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
pub struct Manager {
    pub difficulty_coefficient: f32,
    pub player_ammo: i32,
    pub stage_number: i32,
}

#[derive(Component)]
pub struct Exit;

#[derive(Component)]
pub struct Bullet {
    pub move_vector: Vec3,
}

#[derive(Component)]
pub struct Ammo;

#[derive(Component)]
pub struct AmmoText;

#[derive(Component)]
pub struct HealthText;
