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
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("OpenSans-Bold.ttf"),
                            font_size: 24.,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("OpenSans-Bold.ttf"),
                            font_size: 24.,
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
