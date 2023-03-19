use bevy::prelude::*;

use super::{music_status, score_text};

pub struct AppUIPlugin;

impl Plugin for AppUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(score_text::ScoreTextPlugin)
            .add_plugin(music_status::MusicStatusPlugin);
    }
}
