use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    components::{
        Ammo, Bullet, CameraFlag, Enemy, EnemyFlock, Exit, Manager, Player, TileCollider,
    },
    enemy::set_magnitude,
    make_new_stage, GameState, TILE_SIZE,
};

use crate::tilemap::{MAP_BLOCK_X, MAP_BLOCK_Y};

const PLAYER_SPEED: f32 = 420.0;
const PLAYER_MAX_SPEED: f32 = 400.0;
const STARTING_PLAYER_AMMO: i32 = 3;
const PLAYER_HEALTH: i32 = 3;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_health)
                .with_system(player_shoot)
                .with_system(camera_follow)
                .with_system(player_exit)
                .with_system(player_controller)
                .with_system(update_bullets)
                .with_system(player_phys_update)
                .with_system(player_ammo_check),
        )
        .add_startup_system(spawn_player);
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
            speed: PLAYER_SPEED,
            health: PLAYER_HEALTH,
            shoot_timer: Timer::from_seconds(1.0, false),
            ammo: STARTING_PLAYER_AMMO,
            velocity: Vec3::splat(0.0),
            acceleration: Vec3::splat(0.0),
            max_speed: PLAYER_MAX_SPEED,
        });
}

pub struct MoveDirections {
    up: Vec3,
    down: Vec3,
    left: Vec3,
    right: Vec3,
}

fn player_controller(
    mut query: Query<(&mut Player, &mut Transform), With<Player>>,
    mut tile_query: Query<
        (&Transform, &TileCollider),
        (With<TileCollider>, Without<Player>, Without<EnemyFlock>),
    >,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, transform) = query.single_mut();

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

    player.acceleration += move_vector;
}

fn player_phys_update(
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    tile_query: Query<
        (&Transform, &TileCollider),
        (With<TileCollider>, Without<Player>, Without<EnemyFlock>),
    >,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player_query.single_mut();

    player.velocity = player.velocity + player.acceleration;
    let friction = player.velocity * -0.01;
    player.velocity = player.velocity + friction;
    player.velocity = Vec3::clamp_length_max(player.velocity, player.max_speed);
    player.velocity[2] = 0.0;

    let wish_pos =
        Vec3::new(player.velocity[0] * time.delta_seconds(), 0.0, 0.0) + transform.translation;
    if !wall_collision_check(wish_pos, &tile_query) {
        transform.translation = wish_pos;
    } else {
        player.velocity[0] = 0.0;
    }

    let wish_pos =
        Vec3::new(0.0, player.velocity[1] * time.delta_seconds(), 0.0) + transform.translation;
    if !wall_collision_check(wish_pos, &tile_query) {
        transform.translation = wish_pos;
    } else {
        player.velocity[1] = 0.0;
    }

    player.acceleration = Vec3::splat(0.0);
    transform.translation[2] = 0.0;
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
            speed: PLAYER_SPEED,
            health: PLAYER_HEALTH,
            shoot_timer: Timer::from_seconds(1.0, false),
            ammo: STARTING_PLAYER_AMMO,
            velocity: Vec3::splat(0.0),
            acceleration: Vec3::splat(0.0),
            max_speed: PLAYER_MAX_SPEED,
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
    mut game_manager_query: Query<&mut Manager, With<Manager>>,
    mut assets: Res<AssetServer>,
    time: Res<Time>,
) {
    // set all the vars up correctl
    // there are some problems with this code that leads to crashes when you bring your cursor
    // outside the window but I can't be bothered to fix that right now
    let (player_position, mut player) = player_query.single_mut();
    let mut game_manager = game_manager_query.single_mut();
    let mut w_width: f32 = 1920.0;
    let mut w_height: f32 = 1080.0;
    let mut mouse_position: Vec2 = Vec2::new(w_width / 2.0, w_height / 2.0);
    for window in windows.iter() {
        w_width = window.width();
        w_height = window.height();
        mouse_position = window.cursor_position().unwrap();
    }

    player.shoot_timer.tick(time.delta());

    if player.shoot_timer.finished() && buttons.pressed(MouseButton::Left) && game_manager.player_ammo > 0 {
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
        game_manager.player_ammo -= 1;
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

fn player_health(
    mut commands: Commands,
    player_query: Query<(Entity, &Player), With<Player>>,
    mut state: ResMut<State<GameState>>,
) {
    let (player, player_vars) = player_query.single();
    if player_vars.health < 1 {
        state
            .set(GameState::GameEnd)
            .expect("Failed to change gamestate.");
        commands.entity(player).despawn();
    }
}

fn player_ammo_check(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    mut fuel_query: Query<(Entity, &Transform), (With<Ammo>, Without<Player>)>,
    mut manager_query: Query<&mut Manager, With<Manager>>,
) {
    let (player_transform, mut player) = player_query.single_mut();
    let mut manager = manager_query.single_mut();

    for (fuel, fuel_transform) in fuel_query.iter_mut() {
        if Vec3::distance(player_transform.translation, fuel_transform.translation)
            < TILE_SIZE * 0.98
        {
            commands.entity(fuel).despawn();
            manager.player_ammo += 3;
        }
    }
}
