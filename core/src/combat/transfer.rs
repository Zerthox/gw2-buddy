use super::agent::Agent;

pub use crate::data::Condition;

/// Error margin for times.
pub const TIME_EPSILON: u32 = 10;

/// Transfer tracking.
#[derive(Debug, Clone)]
pub struct TransferTracker {
    /// Detected transfers.
    transfers: Vec<Transfer>,

    /// Condition removes.
    remove: Vec<Remove>,

    /// Condition applies as transfer candidates.
    apply: Vec<Apply>,
}

impl TransferTracker {
    /// Time to retain candidates.
    pub const RETAIN_TIME: i32 = 100;

    /// Creates a new transfer tracker.
    pub const fn new() -> Self {
        Self {
            transfers: Vec::new(),
            remove: Vec::new(),
            apply: Vec::new(),
        }
    }

    /// Returns an iterator over found condition transfers.
    pub fn found(&self) -> &[Transfer] {
        &self.transfers
    }

    /// Adds a new condition remove.
    pub fn add_remove(&mut self, remove: Remove) {
        self.purge(remove.time);
        if let Some(apply) = Self::find_take(&mut self.apply, |apply| apply.matches(&remove)) {
            log::trace!("Transfer match {remove:?} {apply:?}");
            self.add_transfer(apply)
        } else {
            log::trace!("Transfer candidate {remove:?}");
            self.remove.push(remove)
        }
    }

    /// Adds a new condition apply.
    pub fn add_apply(&mut self, apply: Apply) {
        self.purge(apply.time);
        if let Some(remove) = Self::find_take(&mut self.remove, |remove| apply.matches(remove)) {
            log::trace!("Transfer match {apply:?} {remove:?}");
            self.add_transfer(apply)
        } else {
            log::trace!("Transfer candidate {apply:?}");
            self.apply.push(apply)
        }
    }

    /// Adds a new transfer.
    fn add_transfer(&mut self, apply: Apply) {
        let transfer = Transfer::from(apply);
        if let Some(existing) = self
            .transfers
            .iter_mut()
            .find(|other| transfer.is_group(other))
        {
            existing.stacks += 1;
            log::trace!("Transfer update {existing:?}");
        } else {
            log::trace!("Transfer new {transfer:?}");
            self.transfers.push(transfer)
        }
    }

    /// Find an element matching the predicate and take it out of the [`Vec`].
    fn find_take<T>(vec: &mut Vec<T>, pred: impl FnMut(&T) -> bool) -> Option<T> {
        vec.iter()
            .position(pred)
            .map(|index| vec.swap_remove(index))
    }

    /// Purges old information.
    pub fn purge(&mut self, now: i32) {
        self.remove.retain(|el| Self::check_time(el.time, now));
        self.apply.retain(|el| Self::check_time(el.time, now));
    }

    /// Checks if the time should be kept.
    fn check_time(time: i32, now: i32) -> bool {
        time + Self::RETAIN_TIME >= now
    }
}

impl Default for TransferTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a condition apply.
#[derive(Debug, Clone)]
pub struct Apply {
    /// Time of the apply.
    pub time: i32,

    /// Condition applied.
    pub condi: Condition,

    /// Duration applied.
    pub duration: i32,

    /// Target the condition was applied to.
    pub target: Agent,
}

impl Apply {
    /// Creates a new condition apply.
    pub fn new(time: i32, condi: Condition, duration: i32, target: Agent) -> Self {
        Self {
            time,
            condi,
            duration,
            target,
        }
    }

    /// Check whether the apply matches a remove.
    pub fn matches(&self, remove: &Remove) -> bool {
        self.condi == remove.condi
            && self.duration.abs_diff(remove.duration) < TIME_EPSILON
            && self.time.abs_diff(remove.time) < TIME_EPSILON
    }
}

/// Information about a condition remove.
#[derive(Debug, Clone)]
pub struct Remove {
    /// Time of the remove.
    pub time: i32,

    /// Condition removed.
    pub condi: Condition,

    /// Duration removed.
    pub duration: i32,
}

impl Remove {
    /// Creates a new condition transfer.
    pub fn new(time: i32, condi: Condition, duration: i32) -> Self {
        Self {
            time,
            condi,
            duration,
        }
    }
}

/// Information about a condition transfer.
#[derive(Debug, Clone)]
pub struct Transfer {
    /// Time of the transfer.
    pub time: i32,

    /// Condition transferred.
    pub condi: Condition,

    /// Amount of stacks transferred.
    pub stacks: u32,

    /// Target the condition was transferred to.
    pub target: Agent,
}

impl Transfer {
    /// Check whether the transfers should be grouped.
    pub fn is_group(&self, other: &Self) -> bool {
        self.condi == other.condi
            && self.target == other.target
            && self.time.abs_diff(other.time) < TIME_EPSILON
    }
}

impl From<Apply> for Transfer {
    fn from(apply: Apply) -> Self {
        Self {
            time: apply.time,
            condi: apply.condi,
            stacks: 1,
            target: apply.target,
        }
    }
}
