use crate::data::SKILL_OVERRIDES;
use std::collections::{hash_map::Entry, HashMap};

/// Skill map keeping skill information in memory.
#[derive(Debug, Clone)]
pub struct SkillMap {
    /// Internal skill map.
    map: HashMap<u32, Skill>,

    /// Number of non-placeholder entries.
    cached: usize,
}

impl SkillMap {
    /// Creates a new skill map.
    pub fn new() -> Self {
        Self {
            map: Self::override_entries(),
            cached: 0,
        }
    }

    /// Creates skill override entries.
    fn override_entries() -> HashMap<u32, Skill> {
        SKILL_OVERRIDES
            .iter()
            .cloned()
            .map(|(id, name)| (id, Skill::named(name)))
            .collect()
    }

    /// Returns the number of skill overrides.
    pub fn overrides() -> usize {
        SKILL_OVERRIDES.len()
    }

    /// Returns the number of cached non-placeholder skill entries.
    pub fn cached(&self) -> usize {
        self.cached
    }

    /// Resets the stored skill information.
    pub fn reset(&mut self) {
        self.map = Self::override_entries();
        self.cached = 0;
    }

    /// Returns the skill information for the given id.
    ///
    /// Inserts a placeholder if not present.
    pub fn get(&mut self, id: u32) -> &Skill {
        self.map.entry(id).or_insert_with(|| Skill::unnamed(id))
    }

    /// Returns the skill name for the given id.
    ///
    /// Inserts a placeholder if not present.
    pub fn get_name(&mut self, id: u32) -> &str {
        self.get(id).name.as_str()
    }

    /// Attempts to register a skill while replacing placeholders.
    fn try_replace_with(&mut self, id: u32, create: impl FnOnce() -> Skill) -> &Skill {
        match self.map.entry(id) {
            Entry::Occupied(occupied) => {
                let value = occupied.into_mut();
                if value.is_placeholder {
                    *value = create();
                }
                value
            }
            Entry::Vacant(vacant) => {
                let skill = create();
                if !skill.is_placeholder {
                    self.cached += 1;
                }
                vacant.insert(skill)
            }
        }
    }

    /// Attempts to register a skill.
    pub fn try_register(&mut self, id: u32, skill_name: Option<&str>) -> &Skill {
        self.try_replace_with(id, || Skill::from_combat(id, skill_name))
    }

    /// Attempts to duplicate a skill.
    pub fn try_duplicate(&mut self, id: u32, from: u32) {
        if id != from {
            if let Some(Skill {
                is_placeholder: false,
                name,
            }) = self.map.get(&from)
            {
                let new = Skill::placeholder(name);
                self.try_replace_with(id, || new);
            }
        }
    }
}

/// Information about a skill.
#[derive(Debug, Clone)]
pub struct Skill {
    /// Whether the skill is (possibly) a placeholder.
    pub is_placeholder: bool,

    /// Name of the skill.
    pub name: String,
}

impl Skill {
    /// Creates a new skill from combat.
    ///
    /// Name will fallback to the skill id if not present or empty.
    fn from_combat(id: u32, skill_name: Option<&str>) -> Self {
        match skill_name {
            Some(name) if !name.is_empty() => Self::named(name),
            _ => Self::unnamed(id),
        }
    }

    /// Creates a new named skill.
    fn named(name: &str) -> Self {
        Self {
            is_placeholder: false,
            name: name.into(),
        }
    }

    /// Creates a new unnamed skill.
    fn unnamed(id: u32) -> Self {
        Self {
            is_placeholder: true,
            name: id.to_string(),
        }
    }

    /// Creates a new placeholder skill.
    fn placeholder(name: &str) -> Self {
        Self {
            is_placeholder: true,
            name: name.into(),
        }
    }
}
