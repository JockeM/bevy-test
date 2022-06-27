use std::time::Duration;

use crate::{HEIGHT, WIDTH};
use bevy::{prelude::*, window::CursorMoved, math::vec2};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(PartialEq, Eq)]
enum FireType {
    Normal,
    Shotgun,
}

#[derive(Component)]
struct Weapon {
    fire_timer: Timer,
    fire_type: FireType,
}

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Velocity(f32);

#[derive(Component)]
struct Lifetime(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_startup_system(spawn_crosshair);
        app.add_system(player_move);
        app.add_system(crosshair_move);
        app.add_system(player_fire);
        app.add_system(move_entities);
        app.add_system(lifetime_despawn);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0., 0., 0.5),
            ..default()
        })
        .insert(Player { speed: 128.0 })
        .insert(Weapon {
            fire_timer: Timer::new(Duration::from_millis(100), true),
            fire_type: FireType::Shotgun,
        });
}

fn player_move(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds();
    }
}

fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("crosshair.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Crosshair);
}

fn crosshair_move(
    mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
    mut cursor_event_reader: EventReader<CursorMoved>,
) {
    let mut transform = crosshair_query.single_mut();

    if let Some(event) = cursor_event_reader.iter().last() {
        transform.translation.x = event.position.x - (WIDTH as f32 / 2.);
        transform.translation.y = event.position.y - (HEIGHT as f32 / 2.);
    }
}

fn player_fire(
    mut player_query: Query<(&mut Weapon, &Transform), With<Player>>,
    crosshair_query: Query<&Transform, With<Crosshair>>,
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if !keyboard.pressed(MouseButton::Left) {
        return;
    }

    let (mut weapon, player_transform) = player_query.single_mut();

    weapon.fire_timer.tick(time.delta());

    if weapon.fire_timer.finished() {
        let crosshair_transform = crosshair_query.single();
        let p_trans = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
        let c_trans = Vec2::new(
            crosshair_transform.translation.x,
            crosshair_transform.translation.y,
        );

        // TODO: Create a bullet pool
        if weapon.fire_type == FireType::Normal {
            let dir = (c_trans - p_trans).normalize();

            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("bullet.png"),
                    transform: player_transform.clone(),
                    ..default()
                })
                .insert(Bullet)
                .insert(Direction(dir))
                .insert(Velocity(256.))
                .insert(Lifetime(Timer::new(Duration::from_millis(3000), false)));
        } else {
            for _ in 0..5  {
                // TODO: This needs to be better
                let x = (rand::random::<f32>() - 0.5) as f32;
                let y = (rand::random::<f32>() - 0.5) as f32;

                let c = vec2(c_trans.x + x * 40., c_trans.y + y * 40.);

                let dir = (c - p_trans).normalize();

                commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("bullet.png"),
                    transform: player_transform.clone(),
                    ..default()
                })
                .insert(Bullet)
                .insert(Direction(dir))
                .insert(Velocity(256.))
                .insert(Lifetime(Timer::new(Duration::from_millis(3000), false)));
            }
        }
    }
}

fn move_entities(mut query: Query<(&mut Transform, &Direction, &Velocity)>, time: Res<Time>) {
    for (mut transform, dir, vel) in query.iter_mut() {
        transform.translation += (dir.0 * vel.0 * time.delta_seconds()).extend(0.);
    }
}

fn lifetime_despawn(
    mut commands: Commands,
    mut lifetime_query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in lifetime_query.iter_mut() {
        lifetime.0.tick(time.delta());
        if lifetime.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
