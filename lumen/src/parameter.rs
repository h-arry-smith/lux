use std::fmt::Display;

use serde::{Deserialize, Serialize};

// TODO: A parameter has many options, that need to be built up from time
//       from a file. We need a ParameterBuilder and also a struct or
//       something for reading them from files.

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Param {
    Intensity,
    Pan,
    Tilt,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
}

impl Param {
    pub fn from_string(string: &str) -> Option<Param> {
        match string {
            "intensity" => Some(Param::Intensity),
            "pan" => Some(Param::Pan),
            "tilt" => Some(Param::Tilt),
            "red" => Some(Param::Red),
            "blue" => Some(Param::Blue),
            "green" => Some(Param::Green),
            "cyan" => Some(Param::Cyan),
            "magenta" => Some(Param::Magenta),
            "yellow" => Some(Param::Yellow),
            _ => None,
        }
    }

    pub fn is_color(param: &Param) -> bool {
        // FIXME: This is rubbish, must be a better way to define these.
        let color_params = vec![
            Param::Red,
            Param::Green,
            Param::Blue,
            Param::Cyan,
            Param::Magenta,
            Param::Yellow,
        ];

        color_params.contains(param)
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Parameter {
    min: f64,
    max: f64,
    offset: usize,
}

impl Parameter {
    pub fn new(offset: usize, min: f64, max: f64) -> Self {
        Self { min, max, offset }
    }

    pub fn simple(offset: usize) -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            offset,
        }
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
