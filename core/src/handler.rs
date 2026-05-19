use std::path::PathBuf;

pub use arc_util::settings::Settings;
pub use arcdps::imgui::Ui;

pub trait Handler {
    fn render(&mut self, _ui: &Ui) {}

    fn settings(&self) -> Settings;

    fn skills_path(&self) -> Option<PathBuf>;
}
