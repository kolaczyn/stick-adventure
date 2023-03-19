use bevy::prelude::*;

use crate::common::events::{MusicState, MusicToggledEvent};

use super::constants::WINDOW_PADDING;

pub struct MusicStatusPlugin;

impl Plugin for MusicStatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_music_status)
            .add_system(update_text_on_event_system);
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

fn update_text_on_event_system(
    mut music_status_query: Query<(&MusicStatus, &mut Text)>,
    mut music_status_event_reader: EventReader<MusicToggledEvent>,
) {
    for e in music_status_event_reader.iter() {
        let (_, mut text) = music_status_query.single_mut();
        let new_status = match e.state {
            MusicState::Playing => "playing",
            MusicState::Paused => "muted",
        };
        text.sections[0].value = format!("Music is {}", new_status);
    }
}
