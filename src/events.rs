pub enum MusicState {
    Playing,
    Paused,
}

pub struct MusicToggledEvent {
    pub state: MusicState,
}
