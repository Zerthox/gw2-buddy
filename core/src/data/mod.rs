mod buff;
mod condi;
mod skill_def;
mod skill_info;
mod skill_names;

pub use self::{buff::*, condi::*, skill_def::*, skill_info::*, skill_names::*};

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

/// Skill data.
#[derive(Debug)]
pub struct SkillData {
    /// Mapping of skill ids to data index.
    map: HashMap<u32, usize>,

    /// Skill information data.
    data: Vec<SkillInfo>,
}

impl SkillData {
    /// Creates new skill data with the given data.
    pub fn new(skills: impl IntoIterator<Item = SkillDef>) -> Self {
        let iter = skills.into_iter();
        let (size, _) = iter.size_hint();
        let mut data = Vec::<SkillInfo>::with_capacity(size);
        let mut map = HashMap::with_capacity(size);

        for skill in iter {
            if skill.enabled {
                let index = data.len();
                map.insert(skill.id, index);
                for hit_id in &skill.hit_ids {
                    map.insert(*hit_id, index);
                }
                data.push(skill.into());
            } else if let Some(index) = map.remove(&skill.id) {
                // check if primary id
                if data[index].id == skill.id {
                    // remove other hit ids pointing at same index
                    map.retain(|_, i| *i != index);
                }
            }
        }

        data.shrink_to_fit();
        map.shrink_to_fit();

        Self { map, data }
    }

    /// Creates new skill data with the defaults.
    pub fn with_defaults() -> Self {
        Self::new(Self::iter_defaults())
    }

    /// Returns an iterator over the defaults.
    fn iter_defaults() -> impl Iterator<Item = SkillDef> {
        include!(concat!(env!("OUT_DIR"), "/skills.rs")).into_iter()
    }

    /// Checks whether there is an entry for the skill id.
    ///
    /// This includes disabled definitions.
    pub fn contains(&self, id: u32) -> bool {
        self.map.contains_key(&id)
    }

    /// Retrieves the [`SkillInfo`] corresponding to the skill id.
    ///
    /// This includes disabled definitions.
    pub fn get(&self, id: u32) -> Option<&SkillInfo> {
        self.map.get(&id).and_then(|index| self.data.get(*index))
    }

    /// Attempts to load data from a given file path.
    pub fn try_load(&mut self, path: impl AsRef<Path>) -> Result<usize, LoadError> {
        let file = BufReader::new(File::open(path)?);
        let data: Vec<SkillDef> = serde_yaml::from_reader(file)?;
        let count = data.len();
        *self = Self::new(Self::iter_defaults().chain(data));
        Ok(count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoadError {
    NotFound,
    FailedToRead,
    Invalid,
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::FailedToRead,
        }
    }
}

impl From<serde_yaml::Error> for LoadError {
    fn from(_: serde_yaml::Error) -> Self {
        Self::Invalid
    }
}
