use crate::{fixture::ResolvedFixture, fixture_set::FixtureSet};

pub struct Environment {
    pub fixtures: FixtureSet,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            fixtures: FixtureSet::new(),
        }
    }
}
