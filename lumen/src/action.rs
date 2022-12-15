use crate::{parameter::Param, value::Generator};

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
