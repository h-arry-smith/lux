use std::fmt::Display;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Param {
    Intensity,
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
