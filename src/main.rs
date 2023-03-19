mod common;
pub mod entities;
pub mod systems;
mod ui;

use bevy::prelude::*;
use common::{
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    events::EventsPlugin,
};
use entities::{enemy::EnemyPlugin, player::PlayerPlugin, stick::StickPlugin};
use systems::{enemy_spawner::EnemySpawnerPlugin, music::MusicPlugin, score::ScorePlugin};
use ui::app_ui_plugin::AppUIPlugin;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(get_plugins())
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StickPlugin)
        .add_plugin(MusicPlugin)
        .add_plugin(AppUIPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(EnemySpawnerPlugin)
        .add_plugin(EventsPlugin)
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
