use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(startup);
        app.add_system(text_update_system);
    }
}

#[derive(Component)]
struct FpsText;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("OpenSans-Bold.ttf");
    commands
        .spawn_bundle(TextBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 48.,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 48.,
                            color: Color::BLACK,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}
