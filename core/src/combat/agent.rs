use super::{name_of, Player};
use arc_util::colors::{with_alpha, GREEN, GREY, RED, YELLOW};
use arcdps::{
    evtc::{self, AgentKind},
    exports::{Colors, CoreColor},
    Profession,
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray, VariantNames};

// TODO: show id settings?

/// Information about an agent.
#[derive(Debug, Clone)]
pub struct Agent {
    /// Kind of agent.
    pub kind: AgentKind,

    /// Agent profession.
    pub profession: Profession,

    /// Agent name.
    pub name: String,
}

impl Agent {
    /// Creates a new agent.
    pub fn new(kind: AgentKind, profession: Profession, name: impl Into<String>) -> Self {
        Self {
            kind,
            profession,
            name: name.into(),
        }
    }

    /// Checks whether the agent matches the given species.
    pub fn matches_species(&self, species: Option<u32>) -> bool {
        match (species, self.kind) {
            (Some(species), AgentKind::Npc(id) | AgentKind::Gadget(id)) => id as u32 == species,
            _ => false,
        }
    }

    /// Checks whether the agent is a player.
    pub fn is_player(&self) -> bool {
        matches!(self.kind, AgentKind::Player)
    }

    /// Returns agent profession color.
    pub fn prof_color(&self, colors: &Colors) -> [f32; 4] {
        colors.prof_base(self.profession).unwrap_or(GREY)
    }

    /// Returns friendly agent color.
    pub fn friendly_color(&self, colors: &Colors) -> [f32; 4] {
        let color = if self.is_player() {
            self.prof_color(colors)
        } else {
            colors.core(CoreColor::LightGreen).unwrap_or(GREEN)
        };
        with_alpha(color, 1.0)
    }

    /// Returns enemy agent color.
    pub fn enemy_color(&self, colors: &Colors, fight_target: Option<u32>) -> [f32; 4] {
        let color = if self.is_player() {
            self.prof_color(colors)
        } else if self.matches_species(fight_target) {
            colors.core(CoreColor::LightRed).unwrap_or(RED)
        } else {
            colors.core(CoreColor::LightYellow).unwrap_or(YELLOW)
        };
        with_alpha(color, 1.0)
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        match (self.kind, other.kind) {
            (AgentKind::Player, AgentKind::Player) => self.name == other.name,
            (AgentKind::Npc(id1), AgentKind::Npc(id2))
            | (AgentKind::Gadget(id1), AgentKind::Gadget(id2)) => id1 == id2,
            _ => false,
        }
    }
}

impl From<&evtc::Agent> for Agent {
    fn from(agent: &evtc::Agent) -> Self {
        Self::new(agent.kind(), agent.prof.into(), name_of(agent))
    }
}

impl From<&Player> for Agent {
    fn from(player: &Player) -> Self {
        Self::new(AgentKind::Player, player.prof, player.name.clone())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    VariantArray,
    VariantNames,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum AgentFilter {
    All,
    Players,
    NPCs,
}

impl AgentFilter {
    pub fn matches(&self, agent: &Agent) -> bool {
        match self {
            Self::All => true,
            Self::Players => agent.is_player(),
            Self::NPCs => !agent.is_player(),
        }
    }
}
