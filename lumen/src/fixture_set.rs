use std::collections::{hash_map::IterMut, HashMap};

use crate::fixture::Fixture;

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
}
