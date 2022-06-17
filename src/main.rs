mod player;

use bevy::{prelude::*, window::PresentMode};

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
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(hide_mouse)
        .add_plugin(PlayerPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn hide_mouse(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_visibility(false);
}
