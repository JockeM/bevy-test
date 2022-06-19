use crate::{player::Bullet, HEIGHT, WIDTH};
use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component)]
pub struct Enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemies);
        app.add_system(despawn_enemy);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.pressed(KeyCode::J) {
        let x = (rand::random::<f32>() - 0.5) * WIDTH as f32;
        let y = (rand::random::<f32>() - 0.5) * HEIGHT as f32;

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("player.png"),
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            })
            .insert(Enemy);
    }
}

fn despawn_enemy(
    mut commands: Commands,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    bullets_query: Query<&Transform, With<Bullet>>,
) {
    for enemy in enemies_query.iter() {
        for bullet in bullets_query.iter() {
            if let Some(_) = collide(
                enemy.1.translation,
                Vec2::new(16., 16.),
                bullet.translation,
                Vec2::new(2., 2.),
            ) {
                println!("hit");
                commands.entity(enemy.0).despawn();
            }
        }
    }
}
