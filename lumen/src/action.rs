use crate::{parameter::Param, query::Query, value::Generator};

pub struct Apply {
    pub parameter: Param,
    pub generator: Box<dyn Generator>,
}

impl Apply {
    pub fn new(parameter: Param, generator: Box<dyn Generator>) -> Self {
        Self {
            parameter,
            generator,
        }
    }
}

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
