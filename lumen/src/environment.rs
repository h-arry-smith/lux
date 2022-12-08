use crate::{fixture::Fixture, fixture_set::FixtureSet, query::QueryResult};

// FIXME: Remove public access to fixtures
pub struct Environment {
    pub fixtures: FixtureSet,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            fixtures: FixtureSet::new(),
        }
    }

    pub fn query_fixtures<'a>(
        &'a mut self,
        result: &'a QueryResult,
    ) -> impl Iterator<Item = (&usize, &mut Fixture)> {
        self.fixtures
            .iter_mut()
            .filter(|(_, f)| result.contains(&f.id()))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
