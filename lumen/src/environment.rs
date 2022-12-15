use crate::{action::Action, fixture::Fixture, fixture_set::FixtureSet, query::QueryResult};

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

    pub fn run_action(&mut self, action: &Action) {
        for apply_group in action.apply_groups.iter() {
            for (_, fixture) in self.query(&apply_group.query.evaluate(&self.fixtures)) {
                for apply in apply_group.applies.iter() {
                    fixture.apply(apply);
                }
            }
        }
    }

    pub fn query<'a>(
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
