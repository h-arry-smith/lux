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

    pub fn generate_history_and_run_action(&mut self, action: &Action) -> usize {
        let id = self.generate_history();
        self.run_action(action);

        id
    }

    pub fn generate_history(&mut self) -> usize {
        self.record_history()
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

    pub fn revert(&mut self, history_index: usize) {
        dbg!(&history_index);
        if self.history.get(history_index).is_some() {
            // discard all other histories
            self.history = self.history.split_off(history_index);
            // restore the last history
            self.fixtures = self.history.pop().unwrap();
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

    // We return the most recent history ID for the reference of any history
    // generator that wants to know where to return to.
    // The returned history ID always refers to the point in the history before the
    // action was applied.

    // As a return to a point in history discards everything after it, we don't
    // have to worry about those ID's shifting.
    // This may be hopelessly naive, but for now we will use it and in the future
    // we may create some unique identifier for a history
    fn record_history(&mut self) -> usize {
        self.history.push(self.fixtures.clone());
        self.history.len() - 1
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
