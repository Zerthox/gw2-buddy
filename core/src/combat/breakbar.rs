use super::agent::Agent;

/// Information about a defiance damage hit.
#[derive(Debug, Clone)]
pub struct BreakbarHit {
    /// Time of the hit.
    pub time: i32,

    /// Skill causing the hit.
    pub skill: u32,

    /// Defiance damage dealt by the hit.
    ///
    /// Due to decimals this is `10 * damage` compared to regular defiance damage units.
    pub damage: i32,

    /// Agent causing the hit.
    pub attacker: Agent,

    /// Whether the attacker is our character.
    pub is_own: bool,

    /// Target hit.
    pub target: Agent,
}

impl BreakbarHit {
    /// Creates a new breakbar hit.
    pub fn new(
        time: i32,
        skill: u32,
        damage: i32,
        attacker: Agent,
        is_own: bool,
        target: Agent,
    ) -> Self {
        Self {
            time,
            skill,
            damage,
            attacker,
            is_own,
            target,
        }
    }
}
