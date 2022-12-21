use crate::fixture_set::FixtureSet;

pub type HistoryID = usize;

#[derive(Debug)]
pub struct History {
    history: Vec<FixtureSet>,
}

impl History {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    // We return the most recent history ID for the reference of any history
    // generator that wants to know where to return to.
    // The returned history ID always refers to the point in the history before the
    // action was applied.

    // As a return to a point in history discards everything after it, we don't
    // have to worry about those ID's shifting.
    // This may be hopelessly naive, but for now we will use it and in the future
    // we may create some unique identifier for a history
    pub fn record(&mut self, fixture_set: FixtureSet) -> HistoryID {
        self.history.push(fixture_set);
        self.history.len() - 1
    }

    pub fn revert(&mut self, history_index: usize) -> Option<FixtureSet> {
        if self.history.get(history_index).is_some() {
            // discard all other histories
            self.history = self.history.split_off(history_index);
            // restore the last history
            Some(self.history.pop().unwrap())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.history.clear()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
