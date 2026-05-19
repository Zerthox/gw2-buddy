use buddy::{Buddy, Handler, Settings};
use nexus::gui::{RenderType, register_render, render};
use std::sync::{LazyLock, Mutex};

static ADDON: LazyLock<Mutex<Buddy<NexusAddon>>> =
    LazyLock::new(|| Mutex::new(Buddy::new(NexusAddon)));

struct NexusAddon;

impl Handler for NexusAddon {
    fn settings(&self) -> Settings {
        todo!()
    }

    fn skills_path(&self) -> Option<std::path::PathBuf> {
        None
    }
}

nexus::export! {
    name: "Buddy",
    signature: -0x74c13713,
    load: || {
        register_render(RenderType::Render, render!(|ui| {
            ADDON.lock().unwrap().render_windows(ui);
        })).revert_on_unload();

        register_render(RenderType::OptionsRender, render!(|ui| {
            ADDON.lock().unwrap().render_settings(ui);
        })).revert_on_unload();
    },
}
