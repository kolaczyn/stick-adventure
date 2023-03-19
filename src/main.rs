mod collider;
mod music;
mod player;
mod score;
mod stick;
mod ui;

use bevy::prelude::*;
use stick::StickPickedEvent;
use ui::app_ui_plugin::AppUIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(score::ScorePlugin)
        .add_plugin(stick::StickPlugin)
        .add_plugin(music::MusicPlugin)
        .add_plugin(AppUIPlugin)
        .add_startup_system(setup)
        .add_event::<StickPickedEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
