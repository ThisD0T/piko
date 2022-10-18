use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: u32,
}

#[derive(Component)]
pub struct CameraFlag;

#[derive(Component)]
pub struct TileCollider;
