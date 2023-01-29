use std::collections::HashMap;

use crate::{fixture::ParameterMap, parameter::Param, value::Values};

#[derive(Debug, Clone, Copy)]
pub enum Colorspace {
    RGB,
}

impl Colorspace {
    pub fn detect(_parameters: &ParameterMap) -> Colorspace {
        // TODO: Detect colorspace
        Colorspace::RGB
    }

    pub fn params_for_colorspace(colorspace: &Colorspace) -> Vec<Param> {
        match colorspace {
            Colorspace::RGB => vec![Param::Red, Param::Green, Param::Blue],
        }
    }
}

#[derive(Debug)]
pub struct Color {
    pub values: HashMap<Param, Values>,
    colorspace: Colorspace,
}

impl Color {
    pub fn new(colorspace: Colorspace) -> Self {
        Self {
            values: HashMap::new(),
            colorspace,
        }
    }

    pub fn convert_to(self, _target_colorspace: &Colorspace) -> Self {
        // TODO: Do the actual color conversion
        self
    }

    pub fn set(&mut self, parameter: Param, value: Values) {
        self.values.insert(parameter, value);
    }

    pub fn get_value(&self, parameter: &Param) -> Option<&Values> {
        self.values.get(parameter)
    }

    pub fn values(&self) -> &HashMap<Param, Values> {
        &self.values
    }
}
