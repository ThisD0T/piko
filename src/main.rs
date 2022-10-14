use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode};

mod components;
use components::CameraFlag;

mod ascii;
use ascii::AsciiPlugin;

mod tilemap;
use tilemap::TileMapPlugin;

mod player;
use player::PlayerPlugin;

pub const TILE_SIZE: f32 = 15.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .insert_resource(WindowDescriptor {
            title: "Piko".to_string(),
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize;

    commands.spawn_bundle(camera)
        .insert(CameraFlag);
}
