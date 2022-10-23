use bevy::{
    prelude::*, render::camera::ScalingMode, render::texture::ImageSettings, window::WindowMode,
};

mod lib;

mod debug;
use crate::debug::DebugPlugin;

mod components;
use components::{CameraFlag, Manager};

mod gameobject;
use gameobject::GameObjectPlugin;

mod ascii;
use ascii::{AsciiPlugin, AsciiSheet};

mod tilemap;
use tilemap::{generate_map, TileMapPlugin};

mod player;
use player::{respawn_player, PlayerPlugin};

mod enemy;
use enemy::EnemyPlugin;

pub const TILE_SIZE: f32 = 15.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(WindowDescriptor {
            title: "Piko".to_string(),
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(GameObjectPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize;

    commands.spawn_bundle(camera).insert(CameraFlag);
}

fn make_new_stage(
    mut commands: Commands,
    mut ascii: Res<AsciiSheet>,
    mut entities_query: Query<Entity, Without<Manager>>,
    mut assets: Res<AssetServer>,
) {
    for entity in entities_query.iter_mut() {
        commands.entity(entity).despawn();
    }
    generate_map(&mut commands, &mut ascii, &mut assets);
    respawn_player(&mut commands, &mut ascii);
    spawn_camera(commands);
}
