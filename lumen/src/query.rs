use std::collections::HashSet;

use crate::{fixture::FixtureID, fixture_set::FixtureSet};

use self::query_builder::Step;

pub mod query_builder;

pub type QueryResult = HashSet<FixtureID>;

#[derive(Debug)]
pub struct Query {
    steps: Vec<Step>,
}

impl Query {
    pub fn evaluate(&self, fixtures: &FixtureSet) -> QueryResult {
        let mut found = QueryResult::new();

        for step in self.steps.iter() {
            match step {
                Step::All => Self::all(&fixtures, &mut found),
                Step::Even => Self::even(&mut found),
                Step::Id(id) => Self::id(id, &fixtures, &mut found),
            }
        }

        found
    }

    fn all(fixtures: &FixtureSet, found: &mut QueryResult) {
        for id in fixtures.all_ref().map(|(_, f)| f.id()) {
            found.insert(id);
        }
    }

    fn even(found: &mut QueryResult) {
        let ids_to_remove: Vec<FixtureID> =
            found.iter().filter(|id| *id % 2 == 1).cloned().collect();
        for id in ids_to_remove {
            found.remove(&id);
        }
    }

    fn id(id: &FixtureID, fixtures: &FixtureSet, found: &mut QueryResult) {
        match fixtures.get(&id) {
            Some(fixture) => {
                found.insert(fixture.id());
            }
            None => {}
        }
    }
}

// TODO: Integration test all of these query steps, and combinations of them
//       Reason no unit testing, because of the mutable ref to found,

// TODO: The mut ref its a bit nasty. Maybe there is a better way to architect this.
