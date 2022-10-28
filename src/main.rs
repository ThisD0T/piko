use bevy::{
    prelude::*, render::camera::ScalingMode, render::texture::ImageSettings, window::WindowMode,
};

mod lib;

mod components;
use components::{CameraFlag, MainMenuFlag, Manager};

mod gameobject;
use gameobject::GameObjectPlugin;

mod ascii;
use ascii::{AsciiPlugin, AsciiSheet};

mod ui;
use ui::{make_text_bundle, setup_ui, UiPlugin};

mod tilemap;
use tilemap::{generate_map, TileMapPlugin};

mod player;
use player::{respawn_player, PlayerPlugin};

mod enemy;
use enemy::EnemyPlugin;

mod colourscheme;
use colourscheme::{generate_colourscheme, ColourPlugin, ColourScheme};

// mod colourscheme;
// use colourscheme::ColourPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Playing,
    GameEnd,
    OpeningMenu,
}

pub const TILE_SIZE: f32 = 25.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_state(GameState::OpeningMenu)
        .insert_resource(WindowDescriptor {
            title: "Piko".to_string(),
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, game_manager_setup)
        .add_startup_system(main_menu_setup)
        .add_system_set(SystemSet::on_update(GameState::OpeningMenu).with_system(main_menu))
        .add_plugins(DefaultPlugins)
        .add_plugin(ColourPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(GameObjectPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn main_menu_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    colours: Res<ColourScheme>,
    windows: Res<Windows>,
) {
    for window in windows.iter() {

    let start_text = make_text_bundle(
        &mut commands,
        &assets,
        30.0,
        "Press Return to Start Piko!".to_string(),
        colours.colour_0,
        Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(40.0),
                ..default()
            },
            ..default()
        },
    );
    commands.entity(start_text).insert(MainMenuFlag);

    let logo = commands.spawn().id();
    commands.entity(logo).insert_bundle(SpriteBundle{
        sprite: Sprite {
            custom_size: Some(Vec2::splat(window.width() * 0.20)),
            ..default()
        },
        texture: assets.load("title_logo.png"),
        transform: Transform{
            translation: Vec3::new(0.0, 0.0, 90.0),
            ..default()
        },
        ..default()
        
    })
        .insert(MainMenuFlag)
        .insert(Name::new("Logo"));

    let background_image = commands.spawn_bundle(SpriteBundle{
        sprite: Sprite {
            custom_size: Some(Vec2::splat(window.width() * 100.0)),
            ..default()
        },
        texture: assets.load("background.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 40.0),
            ..default()
        },
        ..default()
    })
        .insert(MainMenuFlag)
        .insert(Name::new("Background image"));
    }
}

fn main_menu(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut menu_query: Query<Entity, With<MainMenuFlag>>,
    mut ascii: Res<AsciiSheet>,
    mut entities_query: Query<Entity, Without<Manager>>,
    mut assets: Res<AssetServer>,
    colours: Res<ColourScheme>,
    mut manager_query: Query<&mut Manager, With<Manager>>,
) {
    if keys.just_pressed(KeyCode::Return) {
        for menu_element in menu_query.iter_mut() {
            commands.entity(menu_element).despawn();
        }
        make_new_stage(commands, ascii, entities_query, assets, colours, &mut manager_query);
        state
            .set(GameState::Playing)
            .expect("Failed to change game state.");
    }
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
    colours: Res<ColourScheme>,
    mut manager_query: &mut Query<&mut Manager, With<Manager>>,
    // mut manager_query: Query<&mut Manager, With<Manager>>,
) {
    generate_colourscheme(&mut commands);
    let mut manager = manager_query.single_mut();
    manager.stage_number += 1;
    for entity in entities_query.iter_mut() {
        commands.entity(entity).despawn();
    }
    setup_ui(&mut commands, &assets);
    generate_map(
        &mut commands,
        &mut ascii,
        &mut assets,
        &colours,
        &mut manager_query,
    );
    respawn_player(&mut commands, &mut ascii, &colours);
    spawn_camera(commands);
}

fn game_manager_setup(mut commands: Commands) {
    let game_manager = commands.spawn().id();
    commands.entity(game_manager).insert(Manager {
        difficulty_coefficient: 0.1,
        player_ammo: 3,
        stage_number: 1,
    });
}
