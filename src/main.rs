mod player;
mod score;
mod stick;

use bevy::prelude::*;
use player::{check_player_stick_collision_system, move_player_system, setup_player};
use score::{setup_score, setup_score_text, update_score_text_system};
use stick::{setup_sticks, spawn_new_stick, StickPickedEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_player)
        .add_startup_system(setup_sticks)
        .add_startup_system(setup_score)
        .add_startup_system(setup_score_text)
        .add_system(move_player_system)
        .add_system(check_player_stick_collision_system)
        .add_system(update_score_text_system)
        .add_system(spawn_new_stick)
        .add_event::<StickPickedEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
