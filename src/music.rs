use bevy::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_music)
            .add_system(mute_music_system);
    }
}

#[derive(Resource)]
struct MusicController(Handle<AudioSink>);

fn setup_music(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = assets_server.load("music/elevator-music.ogg");
    // TODO make it play in a loop
    let handle = audio_sinks.get_handle(audio.play(music));
    commands.insert_resource(MusicController(handle));
    println!("Press M to mute music");
}

fn mute_music_system(
    music_controller: ResMut<MusicController>,
    audio_sinks: Res<Assets<AudioSink>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if let Some(audio_sink) = audio_sinks.get(&music_controller.0) {
            let current_volume = audio_sink.volume();
            let new_volume = if current_volume > 0.0 { 0.0 } else { 1.0 };
            audio_sink.set_volume(new_volume);
        }
    }
}
