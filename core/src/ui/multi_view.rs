use super::{
    breakbar_log::{BreakbarLog, BreakbarLogProps},
    buff_log::{BuffLog, BuffLogProps},
    cast_log::{CastLog, CastLogProps},
    transfer_log::{TransferLog, TransferLogProps},
};
use crate::{
    combat::{skill::SkillMap, CombatData},
    data::SkillData,
    history::History,
};
use arc_util::{
    settings::HasSettings,
    ui::{Component, Windowable},
};
use arcdps::imgui::{ChildWindow, TabBar, TabItem, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MultiView {
    pub casts: CastLog,
    pub buffs: BuffLog,
    pub breakbars: BreakbarLog,
    pub transfers: TransferLog,
}

impl MultiView {
    pub fn new() -> Self {
        Self {
            casts: CastLog::new(),
            buffs: BuffLog::new(),
            breakbars: BreakbarLog::new(),
            transfers: TransferLog::new(),
        }
    }

    fn scroll_tab(ui: &Ui, name: impl AsRef<str>, render: impl FnOnce()) {
        TabItem::new(name).build(ui, || ChildWindow::new("##scroller").build(ui, render));
    }
}

#[derive(Debug)]
pub struct MultiViewProps<'a> {
    pub skills: &'a mut SkillMap,
    pub data: &'a SkillData,
    pub history: &'a mut History<CombatData>,
}

impl Component<MultiViewProps<'_>> for MultiView {
    fn render(&mut self, ui: &Ui, props: MultiViewProps) {
        let MultiViewProps {
            skills,
            data,
            history,
        } = props;

        TabBar::new("##tabs").build(ui, || {
            Self::scroll_tab(ui, "Casts", || {
                self.casts.render(
                    ui,
                    CastLogProps {
                        skills,
                        data,
                        history,
                    },
                )
            });
            Self::scroll_tab(ui, "Buffs", || {
                self.buffs.render(ui, BuffLogProps { history })
            });
            Self::scroll_tab(ui, "Breakbar", || {
                self.breakbars
                    .render(ui, BreakbarLogProps { skills, history })
            });
            Self::scroll_tab(ui, "Transfer", || {
                self.transfers.render(ui, TransferLogProps { history })
            });
        });
    }
}

impl Default for MultiView {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<MultiViewProps<'_>> for MultiView {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut MultiViewProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Casts Display", || self.casts.render_display(ui));
        ui.menu("Buffs Display", || self.buffs.render_display(ui));
        ui.menu("Breakbar Display", || self.breakbars.render_display(ui));
        ui.menu("Transfer Display", || self.transfers.render_display(ui));
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MultiViewSettings {
    pub casts: <CastLog as HasSettings>::Settings,
    pub buffs: <BuffLog as HasSettings>::Settings,
    pub breakbars: <BreakbarLog as HasSettings>::Settings,
    pub transfers: <TransferLog as HasSettings>::Settings,
}

impl HasSettings for MultiView {
    type Settings = MultiViewSettings;

    const SETTINGS_ID: &'static str = "multi_view";

    fn current_settings(&self) -> Self::Settings {
        MultiViewSettings {
            casts: self.casts.current_settings(),
            buffs: self.buffs.current_settings(),
            breakbars: self.breakbars.current_settings(),
            transfers: self.transfers.current_settings(),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        let Self::Settings {
            casts,
            buffs,
            breakbars,
            transfers,
        } = loaded;
        self.casts.load_settings(casts);
        self.buffs.load_settings(buffs);
        self.breakbars.load_settings(breakbars);
        self.transfers.load_settings(transfers);
    }
}
