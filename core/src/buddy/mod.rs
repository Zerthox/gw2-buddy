mod event;
mod ui;

use crate::{
    Handler,
    combat::{CombatData, player::Player, skill::SkillMap},
    data::{LoadError, SkillData},
    history::History,
    ui::{
        breakbar_log::BreakbarLog, buff_log::BuffLog, cast_log::CastLog, multi_view::MultiView,
        transfer_log::TransferLog,
    },
};
use arc_util::ui::{Window, WindowOptions};
use semver::Version;

/// Buddy version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main instance.
#[derive(Debug)]
pub struct Buddy<T>
where
    T: Handler,
{
    pub handler: T,

    pub skills: SkillMap,
    pub data: SkillData,
    pub data_state: Result<usize, LoadError>,

    pub self_instance_id: Option<u16>,
    pub players: Vec<Player>,
    pub history: History<CombatData>,

    pub multi_view: Window<MultiView>,
    pub cast_log: Window<CastLog>,
    pub buff_log: Window<BuffLog>,
    pub breakbar_log: Window<BreakbarLog>,
    pub transfer_log: Window<TransferLog>,
}

impl<T> Buddy<T>
where
    T: Handler,
{
    /// Creates a new buddy instance.
    pub fn new(handler: T) -> Self {
        let options = WindowOptions {
            width: 350.0,
            height: 450.0,
            ..Default::default()
        };

        Self {
            handler,

            skills: SkillMap::new(),
            data: SkillData::with_defaults(),
            data_state: Err(LoadError::NotFound),

            self_instance_id: None,
            players: Vec::new(),
            history: History::new(10, 5000, true),

            multi_view: Window::with_default("Buddy Multi", options.clone()),
            cast_log: Window::with_default("Buddy Casts", options.clone()),
            buff_log: Window::with_default("Buddy Buffs", options.clone()),
            breakbar_log: Window::with_default(
                "Buddy Breakbar",
                WindowOptions {
                    width: 350.0,
                    height: 450.0,
                    ..Default::default()
                },
            ),
            transfer_log: Window::with_default("Buddy Transfer", options.clone()),
        }
    }

    pub fn load(&mut self) {
        log::info!("v{} load", VERSION);

        // load settings
        let mut settings = self.handler.settings();
        let settings_version: Option<Version> = settings.load_data("version");
        log::info!(
            "Loaded settings from version {}",
            match &settings_version {
                Some(version) => version.to_string(),
                None => "unknown".into(),
            }
        );

        settings.load_component(&mut self.history);
        settings.load_component(&mut self.multi_view);
        settings.load_component(&mut self.cast_log);
        settings.load_component(&mut self.buff_log);
        settings.load_component(&mut self.breakbar_log);

        self.load_data();
    }

    pub fn unload(&mut self) {
        let mut settings = self.handler.settings();

        settings.store_data("version", VERSION);
        settings.store_component(&self.history);
        settings.store_component(&self.multi_view);
        settings.store_component(&self.cast_log);
        settings.store_component(&self.buff_log);
        settings.store_component(&self.breakbar_log);

        settings.save_file();
    }

    pub fn load_data(&mut self) {
        if let Some(path) = self.handler.skills_path()
            && path.exists()
        {
            self.data_state = self.data.try_load(&path);

            if self.data_state.is_ok() {
                log::info!("Loaded custom definitions from \"{}\"", path.display());
            } else {
                log::warn!(
                    "Failed to load custom definitions from \"{}\"",
                    path.display()
                );
            }
        }
    }

    pub fn reset_data(&mut self) {
        self.data = SkillData::with_defaults();
        self.data_state = Err(LoadError::NotFound);
    }
}
