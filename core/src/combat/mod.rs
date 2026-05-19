pub mod agent;
pub mod breakbar;
pub mod buff;
pub mod cast;
pub mod player;
pub mod skill;
pub mod transfer;

pub use self::agent::Agent;
pub use self::player::Player;

use arcdps::evtc::{self, AgentKind};
use breakbar::BreakbarHit;
use buff::BuffApply;
use cast::Cast;
use transfer::TransferTracker;

/// Generates a name with the given parameters.
pub fn process_name(id: usize, kind: AgentKind, name: Option<&str>) -> String {
    match name {
        Some(name) if !name.is_empty() => name.into(),
        _ => match kind {
            AgentKind::Player => format!("Player:{}", id),
            AgentKind::Npc(species) => format!("NPC:{species}",),
            AgentKind::Gadget(species) => format!("Gadget:{species}"),
        },
    }
}

/// Generates a name for the EVTC agent.
pub fn name_of(agent: &evtc::Agent) -> String {
    process_name(agent.id, agent.kind(), agent.name())
}

#[derive(Debug, Clone)]
pub struct CombatData {
    pub casts: Vec<Cast>,
    pub buffs: Vec<BuffApply>,
    pub breakbar: Vec<BreakbarHit>,
    pub transfers: TransferTracker,
}

impl CombatData {
    pub const fn new() -> Self {
        Self {
            casts: Vec::new(),
            buffs: Vec::new(),
            breakbar: Vec::new(),
            transfers: TransferTracker::new(),
        }
    }
}

impl Default for CombatData {
    fn default() -> Self {
        Self::new()
    }
}
