use crate::fixture::FixtureID;

use super::Query;

#[derive(Debug)]
pub enum Step {
    All,
    Even,
    Id(FixtureID),
}

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
}
