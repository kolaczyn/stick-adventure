use bevy::prelude::*;

#[derive(Component)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
struct ScoreText;

pub fn setup_score(mut commands: Commands) {
    commands.spawn((Score { value: 0 },));
}

pub fn setup_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    left: Val::Px(5.0),
                    top: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        ScoreText,
    ));
}

pub fn update_score_text_system(
    score_query: Query<&Score>,
    mut score_text_query: Query<&mut Text>,
) {
    let score = score_query.single();
    println!("Score: {}", score.value);
    let mut score_text = score_text_query.single_mut();
    score_text.sections[0].value = format!("Score: {}", score.value);
}
