use bevy::prelude::*;

use super::constants::WINDOW_PADDING;

pub struct MusicStatusPlugin;

impl Plugin for MusicStatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_music_status);
    }
}

#[derive(Component)]
struct MusicStatus;

fn setup_music_status(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Music is {}", "playing"),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                }],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(WINDOW_PADDING),
                    bottom: Val::Px(WINDOW_PADDING),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        MusicStatus,
    ));
}
