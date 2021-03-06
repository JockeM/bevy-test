mod enemies;
mod fps;
mod player;

use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;

use enemies::EnemiesPlugin;
use fps::FpsDisplayPlugin;
use player::PlayerPlugin;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Test".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            present_mode: PresentMode::Fifo,
            cursor_visible: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_plugin(FpsDisplayPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemiesPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
