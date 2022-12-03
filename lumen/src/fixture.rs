use std::fmt::Debug;
use std::{collections::HashMap, time::Duration};

use crate::{parameter::Param, value::Value, value_generator::Generator};

type ParameterMap = HashMap<Param, Box<dyn Generator>>;

type ResolvedParameterMap = HashMap<Param, Box<dyn Value>>;

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

    pub fn set(&mut self, parameter: Param, generator: Box<dyn Generator>) {
        self.parameters.insert(parameter, generator);
    }

    pub fn resolve(&self, elapsed: Duration) -> ResolvedFixture {
        let mut resolved_fixture = ResolvedFixture::new(self.id);
        for (param, generator) in self.parameters.iter() {
            resolved_fixture.set(*param, Box::new(generator.generate(elapsed)));
        }

        resolved_fixture
    }
}

impl Debug for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Fixture");

        for (parameter, generator) in self.parameters.iter() {
            debug_struct.field(&parameter.to_string(), generator);
        }

        debug_struct.finish()
    }
}

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

    pub fn set(&mut self, parameter: Param, value: Box<dyn Value>) {
        self.parameters.insert(parameter, value);
    }
}

impl Debug for ResolvedFixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Fixture");

        for (parameter, value) in self.parameters.iter() {
            debug_struct.field(&parameter.to_string(), value);
        }

        debug_struct.finish()
    }
}
