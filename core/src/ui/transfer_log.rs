use crate::{
    combat::CombatData,
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::GREY,
    settings::HasSettings,
    ui::{Component, Windowable},
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TransferLog {
    display_time: bool,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl TransferLog {
    pub const fn new() -> Self {
        Self {
            display_time: true,
            scroll: AutoScroll::new(),
        }
    }

    pub fn render_display(&mut self, ui: &Ui) {
        ui.checkbox("Display time", &mut self.display_time);
    }
}

#[derive(Debug)]
pub struct TransferLogProps<'a> {
    pub history: &'a mut History<CombatData>,
}

impl Component<TransferLogProps<'_>> for TransferLog {
    fn render(&mut self, ui: &Ui, props: TransferLogProps) {
        let TransferLogProps { history } = props;

        if let Some(fight) = history.viewed_fight() {
            let colors = exports::colors();
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);

            for transfer in fight.data.transfers.found() {
                if self.display_time {
                    ui.text_colored(grey, format_time(transfer.time));
                    ui.same_line();
                }

                ui.text(transfer.stacks.to_string());

                ui.same_line();
                ui.text(transfer.condi);

                ui.same_line();
                ui.text_colored(
                    transfer.target.enemy_color(&colors, fight.target),
                    &transfer.target.name,
                );
            }
        }

        self.scroll.update(ui);
    }
}

impl Default for TransferLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<TransferLogProps<'_>> for TransferLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut TransferLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for TransferLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "transfer_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}
