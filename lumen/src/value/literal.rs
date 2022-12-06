use crate::{dmx::Dmx, parameter::Parameter};

use super::Value;
use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq)]
pub struct Literal {
    pub value: f32,
}

impl Literal {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Value for Literal {
    fn value(&self) -> f32 {
        self.value
    }

    fn set(&mut self, value: f32) {
        self.value = value
    }

    fn to_dmx(&self, parameter: &Parameter) -> Dmx {
        let difference = parameter.max() - parameter.min();
        let distance_to_min = (parameter.min() - self.value).abs();
        let factor = distance_to_min / difference;

        Dmx::from_factor(factor)
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.02}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_dmx_simple() {
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(Literal::new(0.0).to_dmx(&parameter), Dmx::new(0));
        assert_eq!(Literal::new(50.0).to_dmx(&parameter), Dmx::new(128));
        assert_eq!(Literal::new(100.0).to_dmx(&parameter), Dmx::new(255));
    }

    #[test]
    fn to_dmx_complex() {
        let parameter = Parameter::new(0, -100.0, 100.0);

        assert_eq!(Literal::new(-100.0).to_dmx(&parameter), Dmx::new(0));
        assert_eq!(Literal::new(-50.0).to_dmx(&parameter), Dmx::new(64));
        assert_eq!(Literal::new(0.0).to_dmx(&parameter), Dmx::new(128));
        assert_eq!(Literal::new(100.0).to_dmx(&parameter), Dmx::new(255));
    }
}
