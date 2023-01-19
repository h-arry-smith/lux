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

        for (i, step) in self.steps.iter().enumerate() {
            match step {
                Step::All => {
                    result.extend(fixtures.clone());
                }

                // TODO: The idea expressed here is that if the even / odd step isn't the
                //       first step, then we are reducing the selection, not adding to it.
                //       This works for now, but might not be the right way to think about
                //       this with more complex queries
                Step::Even => {
                    if i == 0 {
                        result.extend(Self::even(&fixtures));
                    } else {
                        result = Self::even(&result);
                    }
                }
                Step::Odd => {
                    if i == 0 {
                        result.extend(Self::odd(&fixtures));
                    } else {
                        result = Self::odd(&result);
                    }
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

    fn odd(fixtures: &QueryResult) -> QueryResult {
        fixtures.iter().filter(|id| *id % 2 != 0).cloned().collect()
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
