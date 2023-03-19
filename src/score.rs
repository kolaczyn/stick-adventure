use bevy::prelude::*;

use crate::stick::StickPickedEvent;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score)
            .add_system(update_score_on_collision_system);
    }
}

#[derive(Component)]
pub struct Score {
    pub value: u32,
}

fn setup_score(mut commands: Commands) {
    commands.spawn((Score { value: 0 },));
}

fn update_score_on_collision_system(
    mut stick_picked_events: EventReader<StickPickedEvent>,
    mut score_query: Query<&mut Score>,
) {
    if stick_picked_events.iter().next().is_some() {
        score_query.single_mut().value += 1;
    }
}
