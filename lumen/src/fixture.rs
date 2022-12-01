use std::{collections::HashMap, time::Duration};

use crate::{parameter::Param, value::Literal, value_generator::Generator};

type ParameterMap = HashMap<Param, Box<dyn Generator>>;
type ResolvedParameterMap = HashMap<Param, Literal>;

pub struct Fixture {
    id: usize,
    parameters: ParameterMap,
}

impl Fixture {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parameters: ParameterMap::new(),
        }
    }

    pub fn set<G: Generator + 'static>(&mut self, parameter: Param, generator: Box<G>) {
        self.parameters.insert(parameter, generator);
    }

    pub fn resolve(&self, elapsed: Duration) -> ResolvedFixture {
        let mut resolved_fixture = ResolvedFixture::new(self.id);
        for (param, generator) in self.parameters.iter() {
            resolved_fixture.set(*param, generator.generate(elapsed));
        }

        resolved_fixture
    }
}

#[derive(Debug)]
pub struct ResolvedFixture {
    id: usize,
    parameters: ResolvedParameterMap,
}

impl ResolvedFixture {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parameters: ResolvedParameterMap::new(),
        }
    }

    pub fn set(&mut self, parameter: Param, value: Literal) {
        self.parameters.insert(parameter, value);
    }
}
