use std::{
    collections::{hash_map::IterMut, HashMap},
    time::Duration,
};

use crate::{
    fixture::{Fixture, ResolvedFixture},
    Patch,
};

pub struct FixtureSet {
    fixtures: HashMap<usize, Fixture>,
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

    pub fn all(&mut self) -> IterMut<usize, Fixture> {
        self.fixtures.iter_mut()
    }

    pub fn resolve(&mut self, elapsed: Duration, patch: &Patch) -> HashMap<usize, ResolvedFixture> {
        self.fixtures
            .iter_mut()
            .map(|(i, f)| (*i, f.resolve(elapsed, patch.get_profile(&i))))
            .collect()
    }
}
