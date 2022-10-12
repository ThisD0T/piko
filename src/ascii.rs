use bevy::prelude::*;

pub struct AsciiSheet(Handle<TextureAtlas>);

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0),
        Vec2::splat(0.0),
        );

    let atlas_handle = texture_atlases.add(atlas);
    println!("loaded ascii texture atlas");
    commands.insert_resource(AsciiSheet(atlas_handle))
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    sprite_index: usize,
    color: Color,
    translation: Vec3,
    size: f32,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(sprite_index);
    sprite.color = color;

    sprite.custom_size = Some(Vec2::splat(size));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform{
                translation,
                ..default()
            },
            ..default()
        }).id()
}

