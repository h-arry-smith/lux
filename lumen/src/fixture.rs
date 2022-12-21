use crate::action::Apply;
use crate::timecode::time::Time;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::parameter::Param;
use crate::patch::FixtureProfile;
use crate::value::{Generator, Values};

pub type FixtureID = usize;

type ParameterMap = HashMap<Param, Box<dyn Generator>>;

type ResolvedParameterMap = HashMap<Param, Values>;

pub struct Fixture {
    id: FixtureID,
    parameters: ParameterMap,
}

impl Fixture {
    pub fn new(id: FixtureID) -> Self {
        Self {
            id,
            parameters: ParameterMap::new(),
        }
    }

    pub fn id(&self) -> FixtureID {
        self.id
    }

    pub fn apply(&mut self, apply: &Apply) {
        self.set(apply.parameter, apply.generator.clone());
    }

    fn set(&mut self, parameter: Param, generator: Box<dyn Generator>) {
        self.parameters.insert(parameter, generator);
    }

    #[allow(clippy::borrowed_box)]
    pub fn param(&self, parameter: Param) -> Option<&Box<dyn Generator>> {
        self.parameters.get(&parameter)
    }

    pub fn resolve(&mut self, time: &Time, profile: &FixtureProfile) -> ResolvedFixture {
        let mut resolved_fixture = ResolvedFixture::new(self.id);
        for (param, generator) in self.parameters.iter_mut() {
            // It only makes sense for a resolved fixture to have params of the
            // target profile, as abstract params on the fixture will never be
            // converted to dmx.
            if let Some(parameter) = profile.get_parameter(param) {
                resolved_fixture.set(*param, generator.generate(time, parameter));
            }
        }

        resolved_fixture
    }
}

impl Debug for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Fixture");
        debug_struct.field("id", &self.id);

        for (parameter, generator) in self.parameters.iter() {
            debug_struct.field(&parameter.to_string(), generator);
        }

        debug_struct.finish()
    }
}

impl Clone for Fixture {
    fn clone(&self) -> Self {
        let mut fixture = Fixture::new(self.id);

        for (param, generator) in self.parameters.iter() {
            fixture.set(*param, generator.clone())
        }

        fixture
    }
}

pub struct ResolvedFixture {
    id: FixtureID,
    parameters: ResolvedParameterMap,
}

impl ResolvedFixture {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parameters: ResolvedParameterMap::new(),
        }
    }

    pub fn set(&mut self, parameter: Param, value: Values) {
        self.parameters.insert(parameter, value);
    }

    pub fn get_value(&self, parameter: &Param) -> Option<&Values> {
        self.parameters.get(parameter)
    }

    pub fn values(&self) -> Iter<Param, Values> {
        self.parameters.iter()
    }
}

impl Debug for ResolvedFixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Fixture");
        debug_struct.field("id", &self.id);

        for (parameter, value) in self.parameters.iter() {
            debug_struct.field(&parameter.to_string(), value);
        }

        debug_struct.finish()
    }
}
