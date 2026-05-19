use arcdps::{
    Agent,
    evtc::{AgentKind, animation::AnimationProgress},
};

/// Information about a cast (activation).
#[derive(Debug, Clone)]
pub struct Cast {
    /// Time of start event or first registered hit.
    pub time: i32,

    /// Casted skill.
    pub skill: u32,

    /// Current [`CastState`] of the cast.
    pub state: CastState,

    /// Time spent in animation.
    pub duration: i32,

    /// Related hits.
    pub hits: Vec<Hit>,
}

impl Cast {
    /// Creates a new cast from a cast start.
    pub const fn from_start(time: i32, skill: u32, state: CastState) -> Self {
        Self {
            time,
            skill,
            state,
            duration: 0,
            hits: Vec::new(),
        }
    }

    /// Creates a new cast from a cast end.
    pub const fn from_end(time: i32, skill: u32, state: CastState, duration: i32) -> Self {
        Self {
            time,
            skill,
            state,
            duration,
            hits: Vec::new(),
        }
    }

    /// Creates a new cast from an individual hit.
    pub fn from_hit(time: i32, skill: u32, target: &Agent) -> Self {
        Self {
            time,
            skill,
            state: CastState::Pre,
            duration: 0,
            hits: vec![target.into()],
        }
    }

    /// Adds a hit to the cast.
    pub fn hit(&mut self, target: &Agent) {
        self.hits.push(target.into())
    }

    /// Completes the cast.
    pub fn complete(&mut self, skill: u32, result: CastState, duration: i32, time: i32) {
        if let CastState::Pre = self.state {
            self.skill = skill;
            self.time = time - duration;
        }
        self.state = result;
        self.duration = duration;
    }
}

/// Possible cast states.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastState {
    /// Unknown state.
    #[default]
    Unknown,

    /// Cast is ongoing.
    Casting,

    /// Cast started before, start is not registered.
    Pre,

    /// Completed fully.
    Fire,

    /// Cancelled after fire.
    Cancel,

    /// Interrupted before fire.
    Interrupt,
}

impl From<AnimationProgress> for CastState {
    fn from(progress: AnimationProgress) -> Self {
        match progress {
            AnimationProgress::Reset => Self::Fire,
            AnimationProgress::Minimum => Self::Cancel,
            AnimationProgress::Cancel => Self::Interrupt,
            _ => Self::Unknown,
        }
    }
}

/// Information about an individual hit.
#[derive(Debug, Clone)]
pub struct Hit {
    /// Target species.
    pub target: u32,
}

impl From<&Agent> for Hit {
    fn from(target: &Agent) -> Self {
        Self {
            target: match target.kind() {
                AgentKind::Player => 0,
                AgentKind::Npc(species) | AgentKind::Gadget(species) => species as u32,
            },
        }
    }
}
