use crate::{action::Action, query::Query, timecode::time::Time};
use std::collections::{
    hash_map::{Iter, IterMut},
    HashMap, HashSet,
};

use crate::{
    fixture::{Fixture, FixtureID, ResolvedFixture},
    Patch,
};

pub type FixtureMap = HashMap<FixtureID, Fixture>;
pub type ResolvedFixtureMap = HashMap<FixtureID, ResolvedFixture>;

#[derive(Debug)]
pub struct FixtureSet {
    fixtures: FixtureMap,
}

impl FixtureSet {
    pub fn new() -> Self {
        Self {
            fixtures: HashMap::new(),
        }
    }

    // TODO: Proper error handling for when a fixture already exists
    pub fn create_with_id(&mut self, id: FixtureID) {
        let fixture = Fixture::new(id);
        self.fixtures.insert(id, fixture);
    }

    pub fn add_fixture(&mut self, id: FixtureID, fixture: Fixture) {
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

    pub fn resolve(&mut self, time: Time, patch: &Patch) -> ResolvedFixtureMap {
        self.fixtures
            .iter_mut()
            .map(|(i, f)| (*i, f.resolve(&time, patch.get_profile(i))))
            .collect()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, FixtureID, Fixture> {
        self.fixtures.iter_mut()
    }

    pub fn apply_action(&mut self, action: &Action, time: Time) {
        for apply_group in action.apply_groups.iter() {
            for (_, fixture) in self.query(&apply_group.query) {
                for apply in apply_group.applies.iter() {
                    let mut apply = apply.clone();
                    apply.set_start_time(time);
                    fixture.apply(&apply);
                }
            }
        }
    }

    pub fn query(&mut self, query: &Query) -> impl Iterator<Item = (&FixtureID, &mut Fixture)> {
        let result = query.evaluate(&self.ids());
        self.fixtures
            .iter_mut()
            .filter(move |(_, f)| result.contains(&f.id()))
    }

    pub fn clean_clone(&self) -> Self {
        let mut set = FixtureSet::new();
        for (id, _) in self.all_ref() {
            set.create_with_id(*id);
        }

        set
    }

    pub fn fixture_exists(&self, id: &FixtureID) -> bool {
        self.fixtures.contains_key(id)
    }

    pub fn ids(&self) -> HashSet<FixtureID> {
        let mut ids = HashSet::new();
        for (id, _) in self.fixtures.iter() {
            ids.insert(*id);
        }

        ids
    }
}

impl Clone for FixtureSet {
    fn clone(&self) -> Self {
        let mut set = FixtureSet::new();

        for (id, fixture) in self.all_ref() {
            set.add_fixture(*id, fixture.clone());
        }

        set
    }
}

impl Default for FixtureSet {
    fn default() -> Self {
        Self::new()
    }
}
