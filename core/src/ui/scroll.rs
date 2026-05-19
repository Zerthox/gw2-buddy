use arcdps::imgui::Ui;

#[derive(Debug, Clone)]
pub struct AutoScroll {
    last_scroll_max: f32,
}

impl AutoScroll {
    pub const fn new() -> Self {
        Self {
            last_scroll_max: 0.0,
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        let scroll = ui.scroll_y();

        #[allow(clippy::float_cmp)]
        if scroll == self.last_scroll_max {
            ui.set_scroll_here_y_with_ratio(1.0);
        }
        self.last_scroll_max = ui.scroll_max_y();
    }
}

impl Default for AutoScroll {
    fn default() -> Self {
        Self::new()
    }
}
