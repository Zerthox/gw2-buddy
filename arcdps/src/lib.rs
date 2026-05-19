mod plugin;
mod ui;

use plugin::ArcPlugin;

// create exports for arcdps
arcdps::export! {
    name: "Buddy",
    sig: 0x84c13713,
    init: || {
        ArcPlugin::get_buddy().load();
        Ok(())
    },
    release: || ArcPlugin::get_buddy().unload(),
    combat:  ArcPlugin::area_event,
    imgui: ArcPlugin::render,
    options_end: |ui| ArcPlugin::get_buddy().render_settings(ui),
    options_windows: ArcPlugin::render_window_options,
    wnd_filter: ArcPlugin::key_event,
}
