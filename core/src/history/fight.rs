use arcdps::Agent;

/// A fight in the history.
#[derive(Debug, Clone)]
pub struct Fight<T> {
    /// Fight target species.
    pub target: Option<u32>,

    /// Target or fight name.
    pub name: Option<String>,

    /// Start time of the fight.
    pub start: u64,

    /// End time of the fight.
    pub end: Option<u64>,

    /// Associated fight data.
    pub data: T,
}

#[allow(unused)]
impl<T> Fight<T> {
    /// Creates a new fight.
    pub fn new(start: u64, data: T) -> Self
    where
        T: Default,
    {
        Self {
            target: None,
            name: None,
            start,
            end: None,
            data,
        }
    }

    /// Creates a new fight with a given target.
    pub fn with_target(start: u64, species: u32, target: Option<&Agent>, data: T) -> Self
    where
        T: Default,
    {
        let mut fight = Self::new(start, data);
        fight.update_target(species, target);
        fight
    }

    /// Creates a new fight with default data.
    pub fn with_default(start: u64) -> Self
    where
        T: Default,
    {
        Self::new(start, T::default())
    }

    /// Updates the fight target.
    pub fn update_target(&mut self, species: u32, agent: Option<&Agent>) {
        if species > 2 {
            self.target = Some(species);
            self.name = agent
                .and_then(|agent| agent.name())
                .filter(|name| !name.is_empty())
                .map(Into::into);
        } else {
            self.target = None;
            self.name = None;
        }
    }

    /// Checks whether the fight ended.
    pub fn ended(&self) -> bool {
        self.end.is_some()
    }

    /// Returns the fight duration, if the fight ended.
    pub fn duration(&self) -> Option<u64> {
        self.end.map(|end| end - self.start)
    }

    /// Ends the fight, returning the fight duration.
    pub fn end(&mut self, time: u64) -> u64 {
        self.end = Some(time);
        time - self.start
    }

    /// Calculates the timestamp as relative time to the fight start.
    pub fn relative_time(&self, time: u64) -> Option<i32> {
        match self.end {
            Some(end) if time > end => None,
            _ => match time.checked_sub(self.start) {
                Some(rel) => rel.try_into().ok(),
                None => i32::try_from(self.start - time).ok().map(|rel| -rel),
            },
        }
    }
}
