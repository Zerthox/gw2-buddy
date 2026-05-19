use crate::{
    combat::{skill::SkillMap, CombatData},
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::{CYAN, GREY},
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
pub struct BreakbarLog {
    display_time: bool,
    display_others: bool,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl BreakbarLog {
    pub const fn new() -> Self {
        Self {
            display_time: true,
            display_others: false,
            scroll: AutoScroll::new(),
        }
    }

    pub fn render_display(&mut self, ui: &Ui) {
        ui.checkbox("Display time", &mut self.display_time);
        ui.checkbox("Display other players", &mut self.display_others);
    }
}

#[derive(Debug)]
pub struct BreakbarLogProps<'a> {
    pub skills: &'a mut SkillMap,
    pub history: &'a mut History<CombatData>,
}

impl Component<BreakbarLogProps<'_>> for BreakbarLog {
    fn render(&mut self, ui: &Ui, props: BreakbarLogProps) {
        let BreakbarLogProps { skills, history } = props;

        if let Some(fight) = history.viewed_fight() {
            let colors = exports::colors();
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            let blue = colors.core(CoreColor::LightTeal).unwrap_or(CYAN);

            for hit in fight
                .data
                .breakbar
                .iter()
                .filter(|hit| hit.is_own || self.display_others)
            {
                if self.display_time {
                    ui.text_colored(grey, format_time(hit.time));
                    ui.same_line();
                }

                ui.text_colored(blue, format!("{}.{}", hit.damage / 10, hit.damage % 10));

                ui.same_line();
                ui.text(skills.get_name(hit.skill));

                if self.display_others {
                    ui.same_line();
                    ui.text_colored(hit.attacker.friendly_color(&colors), &hit.attacker.name);
                }

                ui.same_line();
                ui.text_colored(
                    hit.target.enemy_color(&colors, fight.target),
                    &hit.target.name,
                );
            }
        }

        self.scroll.update(ui);
    }
}

impl Default for BreakbarLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<BreakbarLogProps<'_>> for BreakbarLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut BreakbarLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for BreakbarLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "breakbar_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}
