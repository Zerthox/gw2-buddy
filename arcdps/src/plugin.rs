use arc_util::{
    settings::Settings,
    update::{Repository, Updater},
};
use arcdps::{Agent, Event, imgui::Ui};
use buddy::{Buddy, Handler};
use std::sync::{LazyLock, Mutex, MutexGuard};

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_buddy.json";

/// Cast skill definition file name.
const SKILLS_FILE: &str = "arcdps_buddy_skills.yml";

#[derive(Debug)]
pub struct ArcPlugin {
    pub updater: Updater,
}

impl ArcPlugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            updater: Updater::new(
                "Buddy",
                Repository::new("zerthox", "arcdps-buddy"),
                buddy::VERSION.parse().unwrap(),
            ),
        }
    }

    pub fn get_buddy() -> MutexGuard<'static, Buddy<Self>> {
        static PLUGIN: LazyLock<Mutex<Buddy<ArcPlugin>>> =
            LazyLock::new(|| Mutex::new(Buddy::new(ArcPlugin::new())));

        PLUGIN.lock().unwrap()
    }

    /// Handles a combat event from area stats.
    pub fn area_event(
        event: Option<&Event>,
        src: Option<&Agent>,
        dst: Option<&Agent>,
        skill_name: Option<&str>,
        _event_id: u64,
        _revision: u64,
    ) {
        Self::get_buddy().area_event(event, src, dst, skill_name);
    }
}

impl Handler for ArcPlugin {
    #[inline]
    fn render(&mut self, ui: &Ui) {
        self.updater.render(ui);
    }

    #[inline]
    fn settings(&self) -> Settings {
        Settings::from_file(SETTINGS_FILE)
    }

    #[inline]
    fn skills_path(&self) -> Option<std::path::PathBuf> {
        Settings::config_path(SKILLS_FILE)
    }
}
