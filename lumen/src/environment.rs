use crate::{action::Action, fixture::Fixture, fixture_set::FixtureSet, query::QueryResult};

// FIXME: Remove public access to fixtures
pub struct Environment {
    pub fixtures: FixtureSet,
    history: Vec<FixtureSet>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            fixtures: FixtureSet::new(),
            history: Vec::new(),
        }
    }

    pub fn run_action(&mut self, action: &Action) {
        self.record_history();

        for apply_group in action.apply_groups.iter() {
            for (_, fixture) in self.query(&apply_group.query.evaluate(&self.fixtures)) {
                for apply in apply_group.applies.iter() {
                    fixture.apply(apply);
                }
            }
        }
    }

    pub fn revert(&mut self) {
        if let Some(old_fixture_set) = self.history.pop() {
            self.fixtures = old_fixture_set;
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

    fn record_history(&mut self) {
        self.history.push(self.fixtures.clone())
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
