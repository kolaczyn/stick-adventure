use bevy::prelude::*;

pub enum MusicState {
    Playing,
    Paused,
}

pub struct MusicToggledEvent {
    pub state: MusicState,
}

pub struct SpawnEnemyEvent;

#[derive(Default)]
pub struct StickPickedEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MusicToggledEvent>()
            .add_event::<SpawnEnemyEvent>()
            .add_event::<SpawnEnemyEvent>()
            .add_event::<StickPickedEvent>();
    }
}
