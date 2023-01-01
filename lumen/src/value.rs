mod convertable;
pub mod generator;
pub use generator::Generator;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
mod literal;
pub use literal::Literal;
mod percentage;
pub use percentage::Percentage;

use crate::{dmx::Dmx, parameter::Parameter};

use self::convertable::{Convertable, Converter};

pub trait Value: Debug {
    fn value(&self) -> f64;
    fn set(&mut self, value: f64);
    fn to_dmx(&self, parameter: &Parameter) -> Dmx;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Values {
    Literal(Literal),
    Percentage(Percentage),
}

impl Values {
    pub fn make_literal(value: f64) -> Values {
        Values::Literal(Literal::new(value))
    }

    pub fn make_percentage(percentage: f64) -> Values {
        Values::Percentage(Percentage::new(percentage))
    }

    pub fn to_dmx(&self, parameter: &Parameter) -> Dmx {
        match self {
            Values::Literal(literal) => literal.to_dmx(parameter),
            Values::Percentage(percentage) => percentage.to_dmx(parameter),
        }
    }
}

impl<T> Convertable<T> for Values {
    fn convert(&self, converter: &dyn Converter<Result = T>) -> T {
        match self {
            Values::Literal(literal) => converter.convert_literal(literal),
            Values::Percentage(percentage) => converter.convert_percentage(percentage),
        }
    }
}
