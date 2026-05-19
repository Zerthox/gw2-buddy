use crate::combat::process_name;
use arcdps::{evtc, Profession};

/// Information about a player.
#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub instance_id: u16,
    pub prof: Profession,
    pub name: String,
}

impl Player {
    /// Creates a new player from a tracking change.
    pub fn from_tracking_change(src: &evtc::Agent, dst: &evtc::Agent) -> Self {
        let kind = dst.kind();
        Self {
            id: src.id,
            instance_id: dst.id as u16,
            prof: dst.prof.into(),
            name: process_name(src.id, kind, src.name()),
        }
    }
}
