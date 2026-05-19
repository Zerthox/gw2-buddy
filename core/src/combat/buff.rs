use super::agent::Agent;

pub use crate::data::Buff;

/// Information about a buff application.
#[derive(Debug, Clone)]
pub struct BuffApply {
    /// Time of the application.
    pub time: i32,

    /// Buff applied.
    pub buff: Buff,

    /// Duration of buff applied.
    pub duration: i32,

    /// Target the buff was applied to.
    pub target: Agent,
}

impl BuffApply {
    /// Creates a new buff apply.
    pub fn new(time: i32, buff: Buff, duration: i32, target: Agent) -> Self {
        Self {
            buff,
            time,
            duration,
            target,
        }
    }
}
