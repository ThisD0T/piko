use bevy::prelude::*;

use crate::components::{HealthText, Player};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_text);
    }
}

fn setup_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let health_text = make_text_bundle(&mut commands, &assets, 30.0, "Health: ".to_string(), Color::GREEN, Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: UiRect{
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    });
    commands.entity(health_text)
        .insert(HealthText);
}

fn make_text_bundle(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    font_size: f32,
    string: String,
    colour: Color,
    style: Style,
) -> Entity{
    let text = commands.spawn().id();
    commands.entity(text).insert_bundle(
        TextBundle::from_sections([
            TextSection::new(
                string,
                TextStyle {
                    font: assets.load("Hack-Regular.ttf"),
                    font_size,
                    color: colour,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: assets.load("Hack-Regular.ttf"),
                font_size,
                color: colour,
                ..default()
            }),
        ])
        .with_style(style),
    );
    text
}

fn update_text(
    mut health_query: Query<&mut Text, With<HealthText>>,
    player_query: Query<(&Player), With<Player>>,
) {
    let mut health_text = health_query.single_mut();
    let player = player_query.single();
    health_text.sections[1].value = format!("{}", player.health);
}

