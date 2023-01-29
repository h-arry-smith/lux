use crate::color::Colorspace;
use std::collections::HashMap;

use crate::{
    address::Address,
    dmx::DmxString,
    fixture::{FixtureID, ResolvedFixture},
    parameter::{Param, Parameter},
    value::Values,
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

    pub fn get_address(&self, id: &FixtureID) -> &Address {
        self.patch.get(id).unwrap().address()
    }
}

impl<'a> Default for Patch<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct FixtureProfile {
    parameters: HashMap<Param, Parameter>,
    colorspace: Option<Colorspace>,
    footprint: usize,
}

impl FixtureProfile {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
            colorspace: None,
            footprint: 0,
        }
    }

    pub fn set_parameter(&mut self, param: Param, parameter: Parameter) {
        // TODO: A paramter can be fine, in which case footprint would be
        //       offset+1 if this is the largest offset we ever found
        if parameter.offset() > self.footprint {
            self.footprint = parameter.offset();
        }

        self.parameters.insert(param, parameter);
    }

    pub fn get_parameter(&self, param: &Param) -> Option<&Parameter> {
        self.parameters.get(param)
    }

    pub fn to_dmx(&self, resolved_fixture: &ResolvedFixture) -> DmxString {
        let mut dmx_string = DmxString::new(self.footprint());

        for (param, parameter) in self.parameters.iter() {
            if let Some(value) = resolved_fixture.get_value(param) {
                dmx_string.set(parameter.offset(), value.to_dmx(parameter));
            } else {
                let default = Values::make_literal(parameter.default());
                dmx_string.set(parameter.offset(), default.to_dmx(parameter));
            }
        }

        dmx_string
    }

    pub fn set_colorspace(&mut self, colorspace: Colorspace) {
        self.colorspace = Some(colorspace);
    }

    pub fn colorspace(&self) -> &Option<Colorspace> {
        &self.colorspace
    }

    fn footprint(&self) -> usize {
        // The footprint is 0 indexed to match the 0 indexing of offsets in
        // parameters, so we return +1 for the correct size.
        self.footprint + 1
    }
}

impl Default for FixtureProfile {
    fn default() -> Self {
        Self::new()
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

    fn address(&self) -> &Address {
        &self.address
    }
}
