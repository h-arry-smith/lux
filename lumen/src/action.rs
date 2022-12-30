use crate::{
    parameter::Param, query::Query, timecode::time::Time, value::generator::BoxedGenerator,
};

#[derive(Debug, Clone)]
pub struct Apply {
    pub parameter: Param,
    pub generator: BoxedGenerator,
}

impl Apply {
    pub fn new(parameter: Param, generator: BoxedGenerator) -> Self {
        Self {
            parameter,
            generator,
        }
    }

    pub fn set_start_time(&mut self, time: Time) {
        self.generator.set_start_time(time);
    }
}

#[derive(Debug, Clone)]
pub struct ApplyGroup {
    pub query: Query,
    pub applies: Vec<Apply>,
}

impl ApplyGroup {
    pub fn new(query: Query) -> Self {
        Self {
            query,
            applies: Vec::new(),
        }
    }

    pub fn add_apply(&mut self, apply: Apply) {
        self.applies.push(apply)
    }
}

#[derive(Debug, Clone)]
pub struct Action {
    pub apply_groups: Vec<ApplyGroup>,
}

impl Action {
    pub fn new() -> Self {
        Self {
            apply_groups: Vec::new(),
        }
    }

    pub fn add_group(&mut self, apply_group: ApplyGroup) {
        self.apply_groups.push(apply_group)
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}
