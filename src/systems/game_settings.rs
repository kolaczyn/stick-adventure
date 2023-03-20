use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ReadSettinsError {
    FileNotFound,
    FileNotReadable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    pub is_music_enabled: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            is_music_enabled: true,
        }
    }
}

impl GameSettings {
    fn try_read_settings() -> Result<GameSettings, ReadSettinsError> {
        let settings =
            std::fs::read_to_string("assets/settings/settings.json").map_err(|e| {
                match e.kind() {
                    std::io::ErrorKind::NotFound => ReadSettinsError::FileNotFound,
                    _ => ReadSettinsError::FileNotReadable,
                }
            })?;
        let settings: GameSettings =
            serde_json::from_str(&settings).map_err(|_| ReadSettinsError::FileNotReadable)?;
        Ok(settings)
    }
    pub fn read_settings() -> GameSettings {
        let settings = Self::try_read_settings();
        match settings {
            Ok(settings) => settings,
            Err(e) => {
                println!(
                    "Error reading settings, using default settings. Reason: {:?}",
                    e
                );
                Self::default()
            }
        }
    }
}
