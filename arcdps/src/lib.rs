use arc_util::{
    ui::Hideable,
    update::{Repository, Updater},
};
use arcdps::{exports, imgui::Ui};
use buddy::Buddy;
use std::sync::{LazyLock, Mutex};

// create exports for arcdps
arcdps::export! {
    name: "Buddy",
    sig: 0x84c13713,
    init: || {
        UPDATER.lock().unwrap().check();
        Buddy::lock().load();
        Ok(())
    },
    release: || Buddy::lock().unload(),
    combat: |event, src, dst, skill, _id, _rev| Buddy::event(event, src, dst, skill),
    imgui,
    options_end: |ui| Buddy::lock().render_settings(ui, true),
    options_windows,
    wnd_filter,
}

static UPDATER: LazyLock<Mutex<Updater>> = LazyLock::new(|| {
    Mutex::new(Updater::unchecked(
        "Buddy",
        Repository::new("zerthox", "arcdps-buddy"),
        Buddy::VERSION.parse().unwrap(),
    ))
});

fn imgui(ui: &Ui, not_loading: bool) {
    let ui_settings = exports::ui_settings();
    if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
        Buddy::lock().render_windows(ui);
        UPDATER.lock().unwrap().render(ui);
    }
}

/// Renders window checkboxes.
fn options_windows(ui: &Ui, option_name: Option<&str>) -> bool {
    if option_name.is_none() {
        let mut buddy = Buddy::lock();
        ui.checkbox("Buddy Multi", buddy.multi_view.visible_mut());
        ui.checkbox("Buddy Casts", buddy.cast_log.visible_mut());
        ui.checkbox("Buddy Buffs", buddy.buff_log.visible_mut());
        ui.checkbox("Buddy Breakbar", buddy.breakbar_log.visible_mut());
        ui.checkbox("Buddy Transfer", buddy.transfer_log.visible_mut());
    }
    false
}

/// Handles a key event.
fn wnd_filter(key: usize, down: bool, prev_down: bool) -> bool {
    if down && !prev_down {
        let Buddy {
            multi_view,
            cast_log,
            buff_log,
            breakbar_log,
            transfer_log,
            ..
        } = &mut *Buddy::lock();

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
