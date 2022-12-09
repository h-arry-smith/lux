pub mod query_builder;

use self::query_builder::Step;
use crate::{fixture::FixtureID, fixture_set::FixtureSet};
use std::collections::HashSet;

pub type QueryResult = HashSet<FixtureID>;

// TODO: The mut ref its a bit nasty. Maybe there is a better way to architect this.

#[derive(Debug)]
pub struct Query {
    steps: Vec<Step>,
}

impl Query {
    pub fn evaluate(&self, fixtures: &FixtureSet) -> QueryResult {
        let mut found = QueryResult::new();

        for step in self.steps.iter() {
            match step {
                Step::All => Self::all(fixtures, &mut found),
                Step::Even => Self::even(&mut found),
                Step::Id(id) => Self::id(id, fixtures, &mut found),
            }
        }

        found
    }

    fn all(fixtures: &FixtureSet, found: &mut QueryResult) {
        for id in fixtures.all_ref().map(|(_, f)| f.id()) {
            found.insert(id);
        }
    }

    // TODO: Even applies only to currently 'found' id's, but is it more semantic
    //       to treat even as a flag, that would apply to future values as well?
    fn even(found: &mut QueryResult) {
        let ids_to_remove: Vec<FixtureID> =
            found.iter().filter(|id| *id % 2 == 1).cloned().collect();
        for id in ids_to_remove {
            found.remove(&id);
        }
    }

    fn id(id: &FixtureID, fixtures: &FixtureSet, found: &mut QueryResult) {
        if let Some(fixture) = fixtures.get(id) {
            found.insert(fixture.id());
        }
    }
}

// TODO: Integration test all of these query steps, and combinations of them
//       Reason no unit testing, because of the mutable ref to found,
