use crate::timecode::time::Time;
use std::collections::{
    hash_map::{Iter, IterMut},
    HashMap,
};

use crate::{
    fixture::{Fixture, FixtureID, ResolvedFixture},
    Patch,
};

pub struct FixtureSet {
    fixtures: HashMap<FixtureID, Fixture>,
}

impl FixtureSet {
    pub fn new() -> Self {
        Self {
            fixtures: HashMap::new(),
        }
    }

    // TODO: Proper error handling for when a fixture already exists
    pub fn create_with_id(&mut self, id: usize) {
        let fixture = Fixture::new(id);
        self.fixtures.insert(id, fixture);
    }

    pub fn all(&mut self) -> IterMut<FixtureID, Fixture> {
        self.fixtures.iter_mut()
    }

    pub fn all_ref(&self) -> Iter<FixtureID, Fixture> {
        self.fixtures.iter()
    }

    pub fn get(&self, id: &FixtureID) -> Option<&Fixture> {
        match self.fixtures.iter().find(|(_, f)| f.id() == *id) {
            Some((_, fixture)) => Some(fixture),
            None => None,
        }
    }

    pub fn resolve(&mut self, time: Time, patch: &Patch) -> HashMap<usize, ResolvedFixture> {
        self.fixtures
            .iter_mut()
            .map(|(i, f)| (*i, f.resolve(&time, patch.get_profile(i))))
            .collect()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, FixtureID, Fixture> {
        self.fixtures.iter_mut()
    }
}

impl Default for FixtureSet {
    fn default() -> Self {
        Self::new()
    }
}
