use std::collections::HashMap;

use crate::{fixture::FixtureID, parameter::Param};

pub struct Patch<'a> {
    patch: HashMap<FixtureID, &'a FixtureProfile>,
}

impl<'a> Patch<'a> {
    pub fn new() -> Self {
        Self {
            patch: HashMap::new(),
        }
    }

    // TODO: Proper error handling for any failure case where a patch does
    //       not succeed
    pub fn patch(&mut self, id: FixtureID, profile: &'a FixtureProfile) {
        self.patch.insert(id, profile);
    }

    pub fn unpatch(&mut self, id: &FixtureID) {
        self.patch.remove(id);
    }

    // TODO: Proper error handling for resolving a fixture that hasn't
    //       been patched
    pub fn get_profile(&self, id: &FixtureID) -> &FixtureProfile {
        self.patch.get(id).unwrap()
    }
}

pub struct FixtureProfile {
    parameters: HashMap<Param, Parameter>,
}

impl FixtureProfile {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }

    pub fn set_parameter(&mut self, param: Param, parameter: Parameter) {
        self.parameters.insert(param, parameter);
    }

    // TODO: Should probably return the option here
    pub fn get_parameter(&self, param: &Param) -> &Parameter {
        self.parameters.get(param).unwrap()
    }
}

pub struct Parameter {
    min: f32,
    max: f32,
}

impl Parameter {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }
}