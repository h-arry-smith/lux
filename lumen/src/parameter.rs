use std::fmt::Display;

// TODO: A parameter has many options, that need to be built up from time
//       from a file. We need a ParameterBuilder and also a struct or
//       something for reading them from files.

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Param {
    Intensity,
}

impl Param {
    pub fn from_string(string: &str) -> Option<Param> {
        match string {
            "intensity" => Some(Param::Intensity),
            _ => None,
        }
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Parameter {
    min: f64,
    max: f64,
    offset: usize,
}

impl Parameter {
    pub fn new(offset: usize, min: f64, max: f64) -> Self {
        Self { min, max, offset }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    // TODO: Parameter should have a custom default
    pub fn default(&self) -> f64 {
        0.0
    }
}
