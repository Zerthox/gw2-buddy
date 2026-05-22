use crate::combat::process_name;
use arcdps::{
    Profession,
    evtc::{self, AgentKind},
};

/// Information about a player.
#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub instance_id: u16,
    pub prof: Profession,
    pub name: String,
}

impl Player {
    /// Creates a new player.
    #[inline]
    pub fn new(id: usize, instance_id: u16, prof: u32, elite: u32, name: Option<&str>) -> Self {
        let kind = AgentKind::new(prof, elite);
        Self {
            id,
            instance_id,
            prof: prof.into(),
            name: process_name(id, kind, name),
        }
    }

    /// Creates a new player from a tracking change.
    #[inline]
    pub fn from_tracking_change(src: &evtc::Agent, dst: &evtc::Agent) -> Self {
        Self::new(src.id, dst.id as u16, dst.prof, dst.elite, src.name())
    }
}
