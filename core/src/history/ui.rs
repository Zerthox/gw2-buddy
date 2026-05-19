use super::History;
use arc_util::colors::GREY;
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};

impl<T> History<T> {
    pub fn render_select(&mut self, ui: &Ui) {
        let colors = exports::colors();

        if self.is_empty() {
            ui.text("No history");
        } else {
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            for (i, fight) in self.fights.iter().enumerate() {
                // TODO: display log start time
                let name = fight.name.as_deref().unwrap_or("Unknown");
                let text = match fight.duration() {
                    Some(duration) => format!("{} ({}s)", name, duration / 1000),
                    None => format!("{} (?s)", name),
                };

                if i == self.viewed {
                    ui.text(text);
                } else {
                    ui.text_colored(grey, text);
                    if ui.is_item_clicked() {
                        self.viewed = i;
                    }
                }
            }
        }
    }
}
