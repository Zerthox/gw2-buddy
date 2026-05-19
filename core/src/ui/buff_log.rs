use crate::{
    combat::{agent::AgentFilter, CombatData},
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::{GREY, YELLOW},
    settings::HasSettings,
    ui::{
        render::{ch_width, enum_combo_array},
        Component, Windowable,
    },
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuffLog {
    display_time: bool,
    display_duration: bool,
    target_filter: AgentFilter,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl BuffLog {
    pub const fn new() -> Self {
        Self {
            display_time: true,
            display_duration: true,
            target_filter: AgentFilter::All,
            scroll: AutoScroll::new(),
        }
    }

    pub fn render_display(&mut self, ui: &Ui) {
        ui.checkbox("Display time", &mut self.display_time);
        ui.checkbox("Display duration", &mut self.display_duration);

        ui.set_next_item_width(ch_width(ui, 16));
        enum_combo_array(ui, "Targets", &mut self.target_filter);
    }
}

#[derive(Debug)]
pub struct BuffLogProps<'a> {
    pub history: &'a mut History<CombatData>,
}

impl Component<BuffLogProps<'_>> for BuffLog {
    fn render(&mut self, ui: &Ui, props: BuffLogProps) {
        let BuffLogProps { history } = props;

        if let Some(fight) = history.viewed_fight() {
            let colors = exports::colors();
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

            for apply in &fight.data.buffs {
                if self.target_filter.matches(&apply.target) {
                    if self.display_time {
                        ui.text_colored(grey, format_time(apply.time));
                        ui.same_line();
                    }

                    ui.text(apply.buff.as_ref());

                    if self.display_duration {
                        ui.same_line();
                        ui.text_colored(
                            yellow,
                            format!("{}.{}s", apply.duration / 1000, apply.duration % 1000),
                        );
                    }

                    ui.same_line();
                    ui.text_colored(apply.target.friendly_color(&colors), &apply.target.name);
                }
            }
        }

        self.scroll.update(ui);
    }
}

impl Default for BuffLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<BuffLogProps<'_>> for BuffLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut BuffLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for BuffLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "buff_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}
