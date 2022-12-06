use std::collections::HashMap;

use crate::{
    address::Address,
    fixture::FixtureID,
    parameter::{Param, Parameter},
};

pub struct Patch<'a> {
    patch: HashMap<FixtureID, ProfileMapping<'a>>,
}

impl<'a> Patch<'a> {
    pub fn new() -> Self {
        Self {
            patch: HashMap::new(),
        }
    }

    // TODO: Proper error handling for any failure case where a patch does
    //       not succeed
    pub fn patch(&mut self, id: FixtureID, address: Address, profile: &'a FixtureProfile) {
        let mapping = ProfileMapping::new(address, profile);
        self.patch.insert(id, mapping);
    }

    pub fn unpatch(&mut self, id: &FixtureID) {
        self.patch.remove(id);
    }

    // TODO: Proper error handling for resolving a fixture that hasn't
    //       been patched
    pub fn get_profile(&self, id: &FixtureID) -> &FixtureProfile {
        self.patch.get(id).unwrap().profile()
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

pub struct ProfileMapping<'a> {
    address: Address,
    profile: &'a FixtureProfile,
}

impl<'a> ProfileMapping<'a> {
    fn new(address: Address, profile: &'a FixtureProfile) -> Self {
        Self { address, profile }
    }

    fn profile(&self) -> &FixtureProfile {
        self.profile
    }
}
