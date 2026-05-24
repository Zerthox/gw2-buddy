use arc_util::{colors::RED, ui::Hideable};
use arcdps::exports::{self, Modifiers};
use buddy::{Buddy, combat::Player};
use nexus::{
    event::{
        ADDON_LOADED,
        arc::{
            AgentUpdate, COMBAT_SQUAD, CombatData, REPLAY_SELF_JOIN, REPLAY_SQUAD_JOIN, SELF_JOIN,
            SELF_LEAVE, SQUAD_JOIN, SQUAD_LEAVE,
        },
    },
    event_consume,
    gui::{RenderType, register_render, render},
    keybind::{Keybind, register_keybind_with_struct},
    keybind_handler, on_unload,
};
use std::{sync::Once, thread, time::Duration};
use windows::{
    System::VirtualKey,
    Win32::UI::Input::KeyboardAndMouse::{MAPVK_VK_TO_VSC, MapVirtualKeyA},
};

nexus::export! {
    name: "Buddy",
    signature: -0x74c13713,
    load,
}

const ARC_SIG: i32 = -0x96b2f;

static ARC: Once = Once::new();

fn load() {
    register_render(
        RenderType::OptionsRender,
        render!(|ui| {
            if ARC.is_completed() {
                Buddy::lock().render_settings(ui, false);
            } else {
                ui.text_colored(RED, "ArcDPS not found");
            }
        }),
    )
    .revert_on_unload();

    if !try_init() {
        ADDON_LOADED
            .subscribe(event_consume!(|sig: Option<&i32>| {
                if sig == Some(&ARC_SIG) {
                    try_init();
                }
            }))
            .revert_on_unload();
    }
}

fn try_init() -> bool {
    if let Err(err) = unsafe { arcdps::search_and_init_arc() } {
        log::error!("Failed to find ArcDPS: {err}");
        return false;
    }
    log::info!("ArcDPS found");

    ARC.call_once(|| {
        Buddy::lock().load();
        on_unload(|| Buddy::lock().unload());

        register_render(
            RenderType::Render,
            render!(|ui| {
                Buddy::lock().render_windows(ui);
            }),
        )
        .revert_on_unload();

        thread::spawn(|| {
            for _ in 0..20 {
                thread::sleep(Duration::from_millis(500));
                let modifiers @ Modifiers {
                    modifier1,
                    modifier2,
                    ..
                } = exports::modifiers();
                if modifier1 != 0 || modifier2 != 0 {
                    log::info!("Migrating keybinds with modifiers {modifier1} {modifier2}");
                    register_keybinds(Some(&modifiers));
                    return;
                }
            }
            log::warn!("Failed to migrate keybinds");
            register_keybinds(None);
        });

        COMBAT_SQUAD
            .subscribe(event_consume!(|data: Option<&CombatData>| {
                if let Some(data) = data
                    && let Some(src) = data.src()
                    && let Some(event) = data.event()
                {
                    Buddy::combat_event(event, src, data.dst(), data.skill_name());
                }
            }))
            .revert_on_unload();

        let handle_join = event_consume!(|data: Option<&AgentUpdate>| {
            if let Some(player) = data {
                Buddy::lock().add_player(
                    Player::new(
                        player.id,
                        player.instance_id as u16,
                        player.prof,
                        player.elite,
                        player.character(),
                    ),
                    player.is_self(),
                );
            }
        });
        SELF_JOIN.subscribe(handle_join).revert_on_unload();
        SQUAD_JOIN.subscribe(handle_join).revert_on_unload();

        let handle_leave = event_consume!(|data: Option<&AgentUpdate>| {
            if let Some(player) = data {
                Buddy::lock().remove_player(player.id);
            }
        });
        SELF_LEAVE.subscribe(handle_leave).revert_on_unload();
        SQUAD_LEAVE.subscribe(handle_leave).revert_on_unload();

        REPLAY_SELF_JOIN.raise(&());
        REPLAY_SQUAD_JOIN.raise(&());
    });

    true
}

fn register_keybinds(modifiers: Option<&Modifiers>) {
    let base = if let Some(modifiers) = modifiers {
        Keybind {
            key: 0,
            alt: has_modifier(modifiers, VirtualKey::Menu.0 as _),
            ctrl: has_modifier(modifiers, VirtualKey::Control.0 as _),
            shift: has_modifier(modifiers, VirtualKey::Shift.0 as _),
        }
    } else {
        Keybind::without_modifiers(0)
    };
    let buddy = Buddy::lock();

    macro_rules! register_keybind {
        ( $id:literal, $window:ident) => {
            register_keybind_with_struct(
                $id,
                keybind_handler!(|_, released| {
                    if !released {
                        Buddy::lock().$window.toggle_visibility()
                    }
                }),
                keybind_for(buddy.$window.options.hotkey, &base),
            )
            .revert_on_unload();
        };
    }

    register_keybind!("BUDDY_MULTI", multi_view);
    register_keybind!("BUDDY_CASTS", cast_log);
    register_keybind!("BUDDY_BUFFS", buff_log);
    register_keybind!("BUDDY_BREAKBAR", breakbar_log);
    register_keybind!("BUDDY_TRANSFER", transfer_log);
}

fn keybind_for(virtual_key: Option<u32>, base: &Keybind) -> Keybind {
    if let Some(virtual_key) = virtual_key {
        let scan_code = unsafe { MapVirtualKeyA(virtual_key, MAPVK_VK_TO_VSC) };
        Keybind {
            key: scan_code as _,
            ..*base
        }
    } else {
        Keybind::without_modifiers(0)
    }
}

fn has_modifier(modifiers: &Modifiers, key: u16) -> bool {
    let Modifiers {
        modifier1,
        modifier2,
        ..
    } = *modifiers;
    modifier1 == key || modifier2 == key
}
