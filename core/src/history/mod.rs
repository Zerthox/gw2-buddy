mod fight;
mod settings;
mod ui;

pub use self::fight::*;
pub use self::settings::*;

use arcdps::Agent;
use std::collections::VecDeque;

/// History of fights.
#[derive(Debug)]
pub struct History<T> {
    pub settings: HistorySettings,
    viewed: usize,
    fights: VecDeque<Fight<T>>,
}

#[allow(unused)]
impl<T> History<T> {
    /// Creates a new history.
    pub const fn new(max_fights: usize, min_duration: u64, discard_at_end: bool) -> Self {
        Self {
            settings: HistorySettings::new(max_fights, min_duration, discard_at_end),
            viewed: 0,
            fights: VecDeque::new(),
        }
    }

    /// Returns the number of fights in the history.
    pub fn len(&self) -> usize {
        self.fights.len()
    }

    /// Returns `true` if the history is empty.
    pub fn is_empty(&self) -> bool {
        self.fights.is_empty()
    }

    /// Returns a reference to the latest fight.
    pub fn latest_fight(&self) -> Option<&Fight<T>> {
        self.fights.front()
    }

    /// Returns a mutable reference to the latest fight.
    pub fn latest_fight_mut(&mut self) -> Option<&mut Fight<T>> {
        self.fights.front_mut()
    }

    /// Returns a mutable reference to the fight at the given index.
    pub fn fight_at(&self, index: usize) -> Option<&Fight<T>> {
        self.fights.get(index)
    }

    /// Returns a mutable reference to the fight at the given index.
    pub fn fight_at_mut(&mut self, index: usize) -> Option<&mut Fight<T>> {
        self.fights.get_mut(index)
    }

    /// Returns an iterator over all fights.
    pub fn all_fights(&self) -> impl Iterator<Item = &Fight<T>> {
        self.fights.iter()
    }

    /// Returns the index of the currently viewed fight.
    pub fn viewed(&self) -> usize {
        self.viewed
    }

    /// Returns a reference to the currently viewed fight.
    pub fn viewed_fight(&self) -> Option<&Fight<T>> {
        self.fight_at(self.viewed)
    }

    /// Returns a mutable reference to the currently viewed fight.
    pub fn viewed_fight_mut(&mut self) -> Option<&mut Fight<T>> {
        self.fight_at_mut(self.viewed)
    }

    /// Updates the viewed index when a fight is added/removed.
    fn update_viewed(&mut self, change: isize) {
        if self.viewed > 0 {
            self.viewed = self.viewed.saturating_add_signed(change);
        }
        if self.viewed >= self.len() {
            self.viewed = 0;
        }
    }

    /// Calculates relative time to start for the latest fight.
    pub fn relative_time(&self, time: u64) -> Option<i32> {
        // TODO: handle timestamp in previous fight
        self.latest_fight()
            .and_then(|fight| fight.relative_time(time))
    }

    /// Returns the latest fight and the relative time to fight start.
    pub fn fight_and_time(&mut self, time: u64) -> Option<(i32, &mut Fight<T>)> {
        // TODO: handle timestamp in previous fight
        self.latest_fight_mut()
            .and_then(|fight| fight.relative_time(time).map(|time| (time, fight)))
    }

    /// Adds a fight to the history.
    pub fn add_fight(&mut self, fight: Fight<T>) {
        if let Some(prev) = self.fights.front() {
            if matches!(prev.duration(), Some(duration) if duration < self.settings.min_duration) {
                self.fights.pop_front();
            }
        }
        if self.len() > self.settings.max_fights {
            self.fights.pop_back();
        }
        self.fights.push_front(fight);
        self.update_viewed(1);
    }

    /// Adds a fight with default data to the history.
    pub fn add_fight_default(&mut self, time: u64)
    where
        T: Default,
    {
        self.add_fight(Fight::new(time, T::default()))
    }

    /// Adds a fight with default data and target information to the history.
    pub fn add_fight_with_target(&mut self, time: u64, species: u32, target: Option<&Agent>)
    where
        T: Default,
    {
        self.add_fight(Fight::with_target(time, species, target, T::default()));
    }

    /// Updates the target for the latest fight.
    ///
    /// If there is no fight present or the latest fight already ended, a new fight with the target is added instead.
    pub fn update_fight_target(&mut self, time: u64, species: u32, target: Option<&Agent>)
    where
        T: Default,
    {
        match self.latest_fight_mut() {
            Some(fight @ Fight { end: None, .. }) => fight.update_target(species, target),
            _ => self.add_fight_with_target(time, species, target),
        }
    }

    /// Ends the latest fight.
    ///
    /// Ignored if the latest fight has already ended.
    pub fn end_latest_fight(&mut self, time: u64) {
        if let Some(fight @ Fight { end: None, .. }) = self.latest_fight_mut() {
            let duration = fight.end(time);
            if self.settings.discard_at_end && duration < self.settings.min_duration {
                self.fights.pop_front();
                self.update_viewed(-1);
            }
        }
    }
}
