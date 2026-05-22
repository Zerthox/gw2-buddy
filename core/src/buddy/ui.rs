use super::Buddy;
use crate::{
    combat::skill::SkillMap,
    data::LoadError,
    ui::{
        breakbar_log::BreakbarLogProps, buff_log::BuffLogProps, cast_log::CastLogProps,
        multi_view::MultiViewProps, transfer_log::TransferLogProps,
    },
};
use arc_util::{
    colors::{GREEN, GREY, RED, YELLOW},
    ui::{Component, render},
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};

impl Buddy {
    /// Callback for standalone UI creation.
    pub fn render(ui: &Ui, not_loading: bool) {
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            Self::lock().render_windows(ui)
        }
    }

    /// Renders standalone UI windows.
    pub fn render_windows(&mut self, ui: &Ui) {
        let Self {
            skills,
            data,
            history,
            ..
        } = self;

        self.multi_view.render(
            ui,
            MultiViewProps {
                skills,
                data,
                history,
            },
        );
        self.cast_log.render(
            ui,
            CastLogProps {
                skills,
                data,
                history,
            },
        );
        self.buff_log.render(ui, BuffLogProps { history });
        self.breakbar_log
            .render(ui, BreakbarLogProps { skills, history });
        self.transfer_log.render(ui, TransferLogProps { history });
    }

    /// Renders settings UI.
    pub fn render_settings(&mut self, ui: &Ui, keybinds: bool) {
        let colors = exports::colors();
        let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
        let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
        let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

        let _style = render::small_padding(ui);

        if keybinds {
            ui.text_colored(grey, "Hotkeys");
            render::input_key(
                ui,
                "##multi-key",
                "Multi",
                &mut self.multi_view.options.hotkey,
            );
            render::input_key(
                ui,
                "##casts-key",
                "Casts",
                &mut self.cast_log.options.hotkey,
            );
            render::input_key(
                ui,
                "##buffs-key",
                "Buffs",
                &mut self.buff_log.options.hotkey,
            );
            render::input_key(
                ui,
                "##breakbar-key",
                "Breakbar",
                &mut self.breakbar_log.options.hotkey,
            );
            render::input_key(
                ui,
                "##transfer-key",
                "Transfer",
                &mut self.transfer_log.options.hotkey,
            );

            ui.spacing();
            ui.spacing();
        }

        ui.text_colored(grey, "Fight history");
        let input_width = 100.0;
        let settings = &mut self.history.settings;

        let mut max_fights = settings.max_fights as _;
        ui.set_next_item_width(input_width);
        if ui
            .input_int("Max fights", &mut max_fights)
            .step(1)
            .step_fast(10)
            .build()
        {
            settings.max_fights = max_fights.try_into().unwrap_or_default();
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Maximum amount of fights saved in the history");
        }

        let mut min_duration = settings.min_duration as _;
        ui.set_next_item_width(input_width);
        if ui
            .input_int("Min duration (ms)", &mut min_duration)
            .step(100)
            .step_fast(1000)
            .build()
        {
            settings.min_duration = min_duration.try_into().unwrap_or_default();
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Minimum duration to keep a fight after ending");
        }

        ui.checkbox("Discard at end", &mut settings.discard_at_end);
        if ui.is_item_hovered() {
            ui.tooltip_text("Whether to discard fights at end of current or start of next");
        }

        ui.spacing();
        ui.spacing();

        // TODO: select data, default only, custom only or both
        ui.text_colored(grey, "Custom data");
        ui.text("Status:");
        ui.same_line();
        match self.data_state {
            Ok(count) => ui.text_colored(green, format!("Loaded {count} entries")),
            Err(LoadError::NotFound) => ui.text_colored(yellow, "Not found"),
            Err(LoadError::FailedToRead) => ui.text_colored(red, "Failed to read file"),
            Err(LoadError::Invalid) => ui.text_colored(red, "Failed to parse"),
        }
        if ui.button("Reload##data") {
            self.load_data();
        }
        ui.same_line_with_spacing(0.0, 5.0);
        if ui.button("Reset##data") {
            self.reset_data();
        }

        ui.spacing();
        ui.spacing();

        ui.text_colored(grey, "Skill cache");
        ui.text(format!("Overrides: {}", SkillMap::overrides()));
        ui.text(format!("Cached: {}", self.skills.cached()));
        if ui.button("Reset##skills") {
            self.skills.reset();
            log::info!("reset skill cache");
        }
    }
}
