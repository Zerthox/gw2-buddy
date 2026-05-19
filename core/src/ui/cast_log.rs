use crate::{
    combat::{cast::CastState, skill::SkillMap, CombatData},
    data::{SkillData, SkillHitCount, SkillHits},
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::{CYAN, GREEN, GREY, RED, YELLOW},
    settings::HasSettings,
    ui::{
        render::{ch_width, enum_combo_array},
        Component, Windowable,
    },
};
use arcdps::{
    exports::{self, Color, Colors, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray, VariantNames};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CastLog {
    display_time: bool,
    display_duration: bool,
    display_hits: HitDisplay,
    only_misses: bool,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl CastLog {
    pub fn new() -> Self {
        Self {
            display_time: true,
            display_duration: true,
            display_hits: HitDisplay::default(),
            only_misses: false,
            scroll: AutoScroll::new(),
        }
    }

    pub fn render_display(&mut self, ui: &Ui) {
        ui.checkbox("Display time", &mut self.display_time);
        ui.checkbox("Display duration", &mut self.display_duration);

        ui.set_next_item_width(ch_width(ui, 16));
        enum_combo_array(ui, "Hits", &mut self.display_hits);

        ui.checkbox("Only misses", &mut self.only_misses);
    }

    fn format_hits(colors: &Colors, hits: usize, info: &SkillHits) -> (Color, String) {
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
        let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
        let blue = colors.core(CoreColor::LightTeal).unwrap_or(CYAN);
        let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

        let color = match info.categorize(hits) {
            SkillHitCount::Miss => red,
            SkillHitCount::Expected => yellow,
            SkillHitCount::Max => green,
            SkillHitCount::OverMax => blue,
        };

        let text = if info.has_hits() {
            format!("{hits}/{}", info.max)
        } else {
            format!("{hits}/X")
        };

        (color, text)
    }
}

#[derive(Debug)]
pub struct CastLogProps<'a> {
    pub skills: &'a mut SkillMap,
    pub data: &'a SkillData,
    pub history: &'a mut History<CombatData>,
}

impl Component<CastLogProps<'_>> for CastLog {
    fn render(&mut self, ui: &Ui, props: CastLogProps) {
        let CastLogProps {
            skills,
            data,
            history,
        } = props;

        if let Some(fight) = history.viewed_fight() {
            let colors = exports::colors();
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
            let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
            let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

            for cast in &fight.data.casts {
                if let Some(info) = data.get(cast.skill) {
                    if self.only_misses {
                        if let Some(hit_info) = &info.hits {
                            if !hit_info.missed(cast.hits.len()) {
                                continue;
                            }
                        }
                    }

                    if self.display_time {
                        ui.text_colored(grey, format_time(cast.time));
                        ui.same_line();
                    }

                    ui.text(skills.get_name(cast.skill));

                    if let Some(hit_info) = &info.hits {
                        if let HitDisplay::Target | HitDisplay::Both = self.display_hits {
                            let target_hits = if let Some(species) = fight.target {
                                cast.hits.iter().filter(|hit| hit.target == species).count()
                            } else {
                                0
                            };
                            let (color, text) = Self::format_hits(&colors, target_hits, hit_info);
                            ui.same_line();
                            ui.text_colored(color, text);
                        }

                        let cleave_hits = cast.hits.len();
                        let (color, text) = Self::format_hits(&colors, cleave_hits, hit_info);
                        match self.display_hits {
                            HitDisplay::Cleave => {
                                ui.same_line();
                                ui.text_colored(color, text)
                            }
                            HitDisplay::Both => {
                                ui.same_line();
                                ui.text_colored(color, format!("({text})"));
                            }
                            _ => {}
                        }
                    }

                    if self.display_duration {
                        let text = format!("{}ms", cast.duration);
                        ui.same_line();
                        match cast.state {
                            CastState::Cancel => ui.text_colored(yellow, text),
                            CastState::Fire => ui.text_colored(green, text),
                            CastState::Interrupt => ui.text_colored(red, text),
                            CastState::Unknown | CastState::Casting | CastState::Pre => {
                                ui.text("?ms")
                            }
                        }
                    }
                }
            }
        }

        self.scroll.update(ui);
    }
}

impl Default for CastLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<CastLogProps<'_>> for CastLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut CastLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for CastLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "cast_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    VariantNames,
    Serialize,
    Deserialize,
    VariantArray,
    AsRefStr,
)]
enum HitDisplay {
    #[default]
    Both,
    Target,
    Cleave,
    None,
}

impl From<usize> for HitDisplay {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Both,
            1 => Self::Target,
            2 => Self::Cleave,
            3 => Self::None,
            _ => Self::default(),
        }
    }
}
