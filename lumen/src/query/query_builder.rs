use crate::fixture::FixtureID;

use super::Query;

#[derive(Debug, Clone)]
pub enum Step {
    All,
    Even,
    Range(FixtureID, FixtureID),
    Id(FixtureID),
    SubQuery(Query),
}

#[derive(Debug, Clone)]
pub struct QueryBuilder {
    steps: Vec<Step>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn build(self) -> Query {
        Query { steps: self.steps }
    }

    pub fn all(mut self) -> Self {
        self.steps.push(Step::All);
        self
    }

    pub fn even(mut self) -> Self {
        self.steps.push(Step::Even);
        self
    }

    pub fn id(mut self, id: usize) -> Self {
        self.steps.push(Step::Id(id));
        self
    }

    pub fn range(mut self, start: usize, end: usize) -> Self {
        self.steps.push(Step::Range(start, end));
        self
    }

    pub fn sub_query(mut self, query: Query) -> Self {
        self.steps.push(Step::SubQuery(query));
        self
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
