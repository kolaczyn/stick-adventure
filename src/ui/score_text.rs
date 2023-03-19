use bevy::prelude::*;

use crate::systems::score::Score;

use super::constants::WINDOW_PADDING;

pub struct ScoreTextPlugin;

impl Plugin for ScoreTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score_text)
            .add_system(update_score_text_system);
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Score: {}", 0),
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
                    top: Val::Px(WINDOW_PADDING),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        ScoreText,
    ));
}

fn update_score_text_system(
    score_query: Query<&Score>,
    mut score_text_query: Query<(&mut Text, &ScoreText)>,
) {
    let score = score_query.single();
    let (mut score_text, _) = score_text_query.single_mut();
    score_text.sections[0].value = format!("Score: {}", score.value);
}
