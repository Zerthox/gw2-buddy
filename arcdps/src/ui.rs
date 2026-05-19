use super::ArcPlugin;
use arc_util::ui::Hideable;
use arcdps::{exports, imgui::Ui};
use buddy::Buddy;

impl ArcPlugin {
    /// Callback for standalone UI creation.
    pub fn render(ui: &Ui, not_loading: bool) {
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            Self::get_buddy().render_windows(ui)
        }
    }

    /// Renders window checkboxes.
    pub fn render_window_options(ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            let Buddy {
                multi_view,
                cast_log,
                buff_log,
                breakbar_log,
                transfer_log,
                ..
            } = &mut *Self::get_buddy();
            ui.checkbox("Buddy Multi", multi_view.visible_mut());
            ui.checkbox("Buddy Casts", cast_log.visible_mut());
            ui.checkbox("Buddy Buffs", buff_log.visible_mut());
            ui.checkbox("Buddy Breakbar", breakbar_log.visible_mut());
            ui.checkbox("Buddy Transfer", transfer_log.visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(key: usize, down: bool, prev_down: bool) -> bool {
        if down && !prev_down {
            let Buddy {
                multi_view,
                cast_log,
                buff_log,
                breakbar_log,
                transfer_log,
                ..
            } = &mut *Self::get_buddy();

            // check for hotkeys
            !multi_view.options.key_press(key)
                && !cast_log.options.key_press(key)
                && !buff_log.options.key_press(key)
                && !breakbar_log.options.key_press(key)
                && !transfer_log.options.key_press(key)
        } else {
            true
        }
    }
}
