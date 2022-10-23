use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    components::{Bullet, CameraFlag, Enemy, EnemyFlock, Exit, Manager, Player, TileCollider},
    enemy::set_magnitude,
    make_new_stage, TILE_SIZE,
};

use crate::tilemap::{MAP_BLOCK_X, MAP_BLOCK_Y};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow)
            .add_system(player_exit)
            .add_system(player_controller)
            .add_system(player_shoot)
            .add_system(update_bullets)
            .add_system(player_health);
    }
}

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        3,
        Color::rgb(0.1, 0.7, 0.4),
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::splat(TILE_SIZE * 0.98),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 220.0,
            health: 2,
            shoot_timer: Timer::from_seconds(1.0, false),
        });
}

pub struct MoveDirections {
    up: Vec3,
    down: Vec3,
    left: Vec3,
    right: Vec3,
}

fn player_controller(
    mut query: Query<(&Player, &mut Transform), With<Player>>,
    mut tile_query: Query<
        (&Transform, &TileCollider),
        (With<TileCollider>, Without<Player>, Without<EnemyFlock>),
    >,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();

    let move_directions = MoveDirections {
        up: Vec3::new(0.0, player.speed * time.delta_seconds(), 0.0),
        down: Vec3::new(0.0, player.speed * time.delta_seconds() * -1.0, 0.0),
        left: Vec3::new((player.speed * time.delta_seconds()) * -1.0, 0.0, 0.0),
        right: Vec3::new(player.speed * time.delta_seconds(), 0.0, 0.0),
    };

    let mut move_vector = Vec3::splat(0.0);

    if keys.pressed(KeyCode::W) {
        move_vector += move_directions.up;
    }
    if keys.pressed(KeyCode::S) {
        move_vector += move_directions.down;
    }

    if keys.pressed(KeyCode::A) {
        move_vector += move_directions.left;
    }
    if keys.pressed(KeyCode::D) {
        move_vector += move_directions.right;
    }

    let target_position = Vec3::new(move_vector[0], 0.0, 0.0) + transform.translation;
    if !wall_collision_check(target_position, &tile_query) {
        transform.translation = target_position;
    }

    let target_position = Vec3::new(0.0, move_vector[1], 0.0) + transform.translation;
    if !wall_collision_check(target_position, &tile_query) {
        transform.translation = target_position;
    }
}

pub fn wall_collision_check(
    target_position: Vec3,
    wall_query: &Query<
        (&Transform, &TileCollider),
        (With<TileCollider>, Without<Player>, Without<EnemyFlock>),
    >,
) -> bool {
    for (wall_transform, wall_collider) in wall_query.iter() {
        let collision = collide(
            target_position,
            Vec2::splat(TILE_SIZE),
            wall_transform.translation,
            wall_collider.size,
        );
        if collision.is_some() {
            return true;
        }
    }
    return false;
}
fn camera_follow(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<CameraFlag>)>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single_mut();

    camera_transform.translation = player_transform.translation;
    camera_transform.translation[2] = 600.0;
}

pub fn respawn_player(mut commands: &mut Commands, ascii: &mut Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        3,
        Color::rgb(0.1, 0.7, 0.4),
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::splat(TILE_SIZE * 0.98),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 280.0,
            health: 100,
            shoot_timer: Timer::from_seconds(1.0, false),
        });
}

fn player_exit(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    assets: Res<AssetServer>,
    entity_query: Query<Entity, Without<Manager>>,
    player_query: Query<&Transform, With<Player>>,
    exit_query: Query<&Transform, (With<Exit>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let exit_transform = exit_query.single();

    if Vec3::distance(player_transform.translation, exit_transform.translation) < TILE_SIZE {
        println!("making new stage");
        make_new_stage(commands, ascii, entity_query, assets);
    }
}

fn player_shoot(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    mut assets: Res<AssetServer>,
    time: Res<Time>,
) {
    // set all the vars up correctl
    // there are some problems with this code that leads to crashes when you bring your cursor
    // outside the window but I can't be bothered to fix that right now
    let (player_position, mut player) = player_query.single_mut();
    let mut w_width: f32 = 1920.0;
    let mut w_height: f32 = 1080.0;
    let mut mouse_position: Vec2 = Vec2::new(w_width / 2.0, w_height / 2.0);
    for window in windows.iter() {
        w_width = window.width();
        w_height = window.height();
        mouse_position = window.cursor_position().unwrap();
    }

    player.shoot_timer.tick(time.delta());

    if player.shoot_timer.finished() {
        if buttons.pressed(MouseButton::Left) {
            let mut shoot_vector = Vec3::new(
                mouse_position[0] - w_width / 2.0,
                mouse_position[1] - w_height / 2.0,
                0.0,
            );
            shoot_vector = set_magnitude(shoot_vector, 10.0);

            make_bullet(
                &mut commands,
                &mut assets,
                player_position.translation,
                shoot_vector,
            );
            player.shoot_timer.reset();
        }
    }
}
// 248~ ascii index
pub fn make_bullet(
    mut commands: &mut Commands,
    mut assets: &mut Res<AssetServer>,
    spawn_position: Vec3,
    move_vector: Vec3,
) {
    let mut bullet = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(6.0)),
            ..default()
        },
        transform: Transform {
            translation: spawn_position,
            ..default()
        },
        texture: assets.load("bullet.png"),
        ..default()
    });
    bullet.insert(Bullet { move_vector });
}

fn update_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bullet), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Bullet>)>,
) {
    for (bullet, mut transform, bullet_vars) in query.iter_mut() {
        transform.translation += bullet_vars.move_vector;
        for (enemy, enemy_transform) in enemy_query.iter_mut() {
            if Vec3::distance(transform.translation, enemy_transform.translation) < TILE_SIZE {
                commands.entity(enemy).despawn();
                commands.entity(bullet).despawn();
            }
        }
    }
}

fn player_health(mut commands: Commands, player_query: Query<(Entity, &Player), With<Player>>) {
    let (player, player_vars) = player_query.single();
    if player_vars.health < 1 {
        commands.entity(player).despawn();
    }
}
