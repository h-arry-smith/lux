pub mod query_builder;

use self::query_builder::Step;
use crate::fixture::FixtureID;
use std::collections::HashSet;

pub type QueryResult = HashSet<FixtureID>;

#[derive(Debug, Clone)]
pub struct Query {
    pub steps: Vec<Step>,
}

impl Query {
    pub fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }

    pub fn evaluate(&self, fixtures: &QueryResult) -> QueryResult {
        let mut result = QueryResult::new();
        let mut fixtures = fixtures.clone();

        for step in self.steps.iter() {
            match step {
                Step::All => {
                    result.extend(fixtures.clone());
                }
                Step::Even => {
                    result.extend(Self::even(&fixtures));
                }
                Step::Id(id) => {
                    result.extend(Self::id(id, &fixtures));
                }
                Step::Range(start, end) => {
                    result.extend(Self::range(start, end, &fixtures));
                }
                Step::SubQuery(query) => {
                    fixtures = query.evaluate(&fixtures);
                }
            };
        }

        result
    }

    fn even(fixtures: &QueryResult) -> QueryResult {
        fixtures.iter().filter(|id| *id % 2 == 0).cloned().collect()
    }

    fn id(id: &FixtureID, fixtures: &QueryResult) -> QueryResult {
        let mut result = QueryResult::new();

        if fixtures.contains(id) {
            result.insert(*id);
        }

        result
    }

    fn range(start: &FixtureID, end: &FixtureID, fixtures: &QueryResult) -> QueryResult {
        fixtures
            .iter()
            .filter(|id| (start..=end).contains(id))
            .cloned()
            .collect()
    }
}
