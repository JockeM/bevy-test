use bevy::{prelude::*, window::CursorMoved};

use crate::{HEIGHT, WIDTH};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub last_shot: u128,
    pub firerate: u16,
}

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct Direction(Vec2);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_startup_system(spawn_crosshair);
        app.add_system(player_move);
        app.add_system(crosshair_move);
        app.add_system(player_fire);
        app.add_system(bullet_move);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::default(),
            ..default()
        })
        .insert(Player {
            speed: 128.0,
            last_shot: 0,
            firerate: 100,
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
            transform: Transform::default(),
            ..default()
        })
        .insert(Crosshair);
}

fn crosshair_move(
    mut crosshair_query: Query<(&Crosshair, &mut Transform)>,
    mut cursor_event_reader: EventReader<CursorMoved>,
) {
    let (_, mut transform) = crosshair_query.single_mut();

    if let Some(event) = cursor_event_reader.iter().last() {
        transform.translation.x = event.position.x - (WIDTH as f32 / 2.);
        transform.translation.y = event.position.y - (HEIGHT as f32 / 2.);
    }
}

fn player_fire(
    mut player_query: Query<(&mut Player, &Transform)>,
    crosshair_query: Query<(&Crosshair, &Transform)>,
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if !keyboard.pressed(MouseButton::Left) {
        return;
    }

    let (mut player, player_transform) = player_query.single_mut();
    let time_since_startup = time.time_since_startup().as_millis();
    if (player.last_shot + player.firerate as u128) < time_since_startup {
        let (_, crosshair_transform) = crosshair_query.single();
        let p_trans = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
        let c_trans = Vec2::new(
            crosshair_transform.translation.x,
            crosshair_transform.translation.y,
        );
        let dir = (c_trans - p_trans).normalize();

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("bullet.png"),
                transform: player_transform.clone(),
                ..default()
            })
            .insert(Bullet)
            .insert(Direction(dir));

        player.last_shot = time_since_startup;
    }
}

const BULLET_SPEED: f32 = 256.0;

fn bullet_move(mut moving: Query<(&mut Transform, &Direction)>, time: Res<Time>) {
    for (mut transform, dir) in moving.iter_mut() {
        transform.translation.x += dir.0.x * time.delta_seconds() * BULLET_SPEED;
        transform.translation.y += dir.0.y * time.delta_seconds() * BULLET_SPEED;
    }
}
