mod collider;
mod common;
pub mod constants;
mod enemy;
pub mod enemy_spawner;
mod events;
mod music;
mod player;
mod score;
mod stick;
mod ui;

use bevy::prelude::*;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use events::MusicToggledEvent;
use stick::StickPickedEvent;
use ui::app_ui_plugin::AppUIPlugin;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(get_plugins())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(score::ScorePlugin)
        .add_plugin(stick::StickPlugin)
        .add_plugin(music::MusicPlugin)
        .add_plugin(AppUIPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(enemy_spawner::EnemySpawnerPlugin)
        .add_event::<StickPickedEvent>()
        .add_event::<MusicToggledEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_plugins() -> bevy::app::PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Stick Adventure".to_string(),
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            ..default()
        }),
        ..default()
    })
}
