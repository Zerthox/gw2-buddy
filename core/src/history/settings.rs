use super::History;
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySettings {
    /// How many fights to keep in the history.
    pub max_fights: usize,

    /// Minimum duration to keep a fight.
    pub min_duration: u64,

    /// Whether to discard the fight at end of current or start of new.
    pub discard_at_end: bool,
}

impl HistorySettings {
    pub const fn new(max_fights: usize, min_duration: u64, discard_at_end: bool) -> Self {
        Self {
            max_fights,
            min_duration,
            discard_at_end,
        }
    }
}

impl<T> HasSettings for History<T> {
    type Settings = HistorySettings;

    const SETTINGS_ID: &'static str = "history";

    fn current_settings(&self) -> Self::Settings {
        self.settings.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.settings = loaded;
    }
}
