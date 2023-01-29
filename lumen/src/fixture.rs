use serde::Serialize;

use crate::action::Apply;
use crate::color::{Color, Colorspace};
use crate::timecode::time::Time;
use crate::value::generator::BoxedGenerator;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::parameter::{Param, Parameter};
use crate::patch::FixtureProfile;
use crate::value::Values;

pub type FixtureID = usize;

pub type ParameterMap = HashMap<Param, Vec<BoxedGenerator>>;

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

    pub fn clear_parameter(&mut self, parameter: &Param) {
        self.parameters.remove(parameter);
    }

    fn set(&mut self, parameter: Param, generator: BoxedGenerator) {
        match self.parameters.get_mut(&parameter) {
            Some(generator_vector) => {
                generator_vector.push(generator);
            }
            None => {
                self.parameters.insert(parameter, vec![generator]);
            }
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_parameter(&self, parameter: Param) -> Option<&Vec<BoxedGenerator>> {
        self.parameters.get(&parameter)
    }

    pub fn parameters(&self) -> &ParameterMap {
        &self.parameters
    }

    pub fn resolve(&mut self, time: &Time, profile: &FixtureProfile) -> ResolvedFixture {
        let mut resolved_fixture = ResolvedFixture::new(self.id);

        self.resolve_main_parameters(&mut resolved_fixture, time, profile);
        self.resolve_color_parameters(&mut resolved_fixture, time, profile);

        resolved_fixture
    }

    fn resolve_main_parameters(
        &mut self,
        resolved_fixture: &mut ResolvedFixture,
        time: &Time,
        profile: &FixtureProfile,
    ) {
        for (param, generators) in self
            .parameters
            .iter_mut()
            .filter(|(p, _g)| !Param::is_color(p))
        {
            // It only makes sense for a resolved fixture to have params of the
            // target profile, as abstract params on the fixture will never be
            // converted to dmx.
            if let Some(parameter) = profile.get_parameter(param) {
                let mut latest_time = Time::at(0, 0, 0, 0);

                for generator in generators {
                    // Resolve any current values with the current parameter value
                    match resolved_fixture.get_value(param) {
                        Some(value) => {
                            generator.resolve(value, time);
                        }
                        None => {
                            generator.resolve(&Values::make_literal(parameter.default()), time);
                        }
                    }

                    // If a generator returns None, we keep the previous value
                    if let Some(value) = generator.generate(time, parameter) {
                        if generator.start_time() >= latest_time {
                            latest_time = generator.start_time();
                            resolved_fixture.set(*param, value);
                        }
                    }
                }
            }
        }
    }

    fn resolve_color_parameters(
        &mut self,
        resolved_fixture: &mut ResolvedFixture,
        time: &Time,
        profile: &FixtureProfile,
    ) {
        if let Some(profile_colorspace) = profile.colorspace() {
            // detect from params which colorspace is being used
            let current_colorspace = Colorspace::detect(&self.parameters);

            // construct a resolved color object from colors that are present from that colorspace
            let mut color = Color::new(current_colorspace);

            for (param, generators) in self.parameters.iter_mut().filter(|(p, _g)| {
                Colorspace::params_for_colorspace(&current_colorspace).contains(p)
            }) {
                if let Some(parameter) = profile.get_parameter(param) {
                    let mut latest_time = Time::at(0, 0, 0, 0);

                    for generator in generators {
                        // Resolve any current values with the current parameter value
                        match color.get_value(param) {
                            Some(value) => {
                                generator.resolve(value, time);
                            }
                            None => {
                                generator.resolve(&Values::make_literal(parameter.default()), time);
                            }
                        }

                        // If a generator returns None, we keep the previous value
                        if let Some(value) = generator.generate(time, parameter) {
                            if generator.start_time() >= latest_time {
                                latest_time = generator.start_time();
                                color.set(*param, value);
                            }
                        }
                    }
                }
            }

            // convert that color object to profile colorspace if needed
            let color = color.convert_to(profile_colorspace);

            // apply those colors to the resolving fixture
            for (param, value) in color.values() {
                resolved_fixture.set(*param, *value)
            }
        }
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

        for (param, generators) in self.parameters.iter() {
            for generator in generators {
                fixture.set(*param, generator.clone())
            }
        }

        fixture
    }
}

#[derive(Serialize)]
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
