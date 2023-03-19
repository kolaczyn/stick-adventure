mod collider;
mod player;
mod score;
mod stick;

use bevy::prelude::*;
use stick::StickPickedEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(score::ScorePlugin)
        .add_plugin(stick::StickPlugin)
        .add_startup_system(setup)
        .add_event::<StickPickedEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
