use crate::patch::Parameter;

use super::{Literal, Value};
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
pub struct Percentage {
    pub percentage: f32,
}

impl Percentage {
    pub fn new(percentage: f32) -> Self {
        Self { percentage }
    }

    pub fn to_literal(&self, parameter: &Parameter) -> Literal {
        let difference = parameter.max() - parameter.min();
        let factor = self.percentage / 100.0;
        let result = parameter.min() + (difference * factor);

        Literal::new(result)
    }
}

impl Value for Percentage {
    fn value(&self) -> f32 {
        self.percentage
    }

    fn set(&mut self, value: f32) {
        self.percentage = value
    }
}

impl Debug for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.02}%", self.percentage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{patch::Parameter, value::Literal};

    #[test]
    fn to_literal() {
        let parameter = Parameter::new(25.0, 75.0);

        assert_eq!(
            Percentage::new(0.0).to_literal(&parameter),
            Literal::new(25.0)
        );

        assert_eq!(
            Percentage::new(50.0).to_literal(&parameter),
            Literal::new(50.0)
        );

        assert_eq!(
            Percentage::new(100.0).to_literal(&parameter),
            Literal::new(75.0)
        );
    }
}
