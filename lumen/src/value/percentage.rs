//! A Percentage Value
//!
//! A percentage value represents a 0-100% range, which is usually evaluated in
//! context with a parameter, to provide a literal value. Though it is also
//! possible to convert it to DMX directly.
//!
//! # Examples
//!
//! ```
//! use lumen::value::{Literal, Percentage};
//! use lumen::parameter::Parameter;
//! let percentage = Percentage::new(50.0);
//! let parameter = Parameter::new(0, 0.0, 50.0);
//!
//! assert_eq!(
//!     percentage.to_literal(&parameter),
//!     Literal::new(25.0)
//! );
//! ```

use crate::{dmx::Dmx, parameter::Parameter};

use super::{Literal, Value};
use std::fmt::Debug;

/// A `Perecentage` type represents a percentage based value.
///
/// A `Percentage` is constructed from a `f64` representing it's percentaage
/// value.
///
/// A `Percentage` implements the [`Value`] trait.
#[derive(Clone, Copy, PartialEq)]
pub struct Percentage {
    pub percentage: f64,
}

impl Percentage {
    /// Creates a new `Percentage` with the supplied percentage value.
    ///
    /// # Examples
    ///
    /// ```
    /// use lumen::value::Percentage;
    /// let percentage = Percentage::new(50.0);
    /// ```
    pub fn new(percentage: f64) -> Self {
        Self { percentage }
    }

    /// Converts a `Percentage` to a `Literal`
    ///
    /// Given a `Parameter` that provides the necessary context for the
    /// conversion, we are able to calculate the `Literal` value and return it.
    ///
    /// # Examples
    ///
    /// ```
    /// use lumen::value::{Literal, Percentage};
    /// use lumen::parameter::Parameter;
    /// let percentage = Percentage::new(50.0);
    /// let parameter = Parameter::new(0, 0.0, 50.0);
    ///
    /// assert_eq!(
    ///     percentage.to_literal(&parameter),
    ///     Literal::new(25.0)
    /// );
    /// ```
    pub fn to_literal(&self, parameter: &Parameter) -> Literal {
        let difference = parameter.max() - parameter.min();
        let result = parameter.min() + (difference * self.factor());

        Literal::new(result)
    }

    fn factor(&self) -> f64 {
        self.percentage / 100.0
    }
}

impl Value for Percentage {
    fn value(&self) -> f64 {
        self.percentage
    }

    fn set(&mut self, value: f64) {
        self.percentage = value
    }

    fn to_dmx(&self, _parameter: &Parameter) -> crate::dmx::Dmx {
        Dmx::from_factor(self.factor())
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
    use crate::{dmx::Dmx, parameter::Parameter, value::Literal};

    #[test]
    fn to_literal() {
        let parameter = Parameter::new(0, 25.0, 75.0);

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

    #[test]
    fn to_dmx_simple() {
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(Percentage::new(0.0).to_dmx(&parameter), Dmx::new(0));
        assert_eq!(Percentage::new(50.0).to_dmx(&parameter), Dmx::new(128));
        assert_eq!(Percentage::new(100.0).to_dmx(&parameter), Dmx::new(255));
    }

    #[test]
    fn to_dmx_complex() {
        let parameter = Parameter::new(0, -100.0, 100.0);

        assert_eq!(Percentage::new(0.0).to_dmx(&parameter), Dmx::new(0));
        assert_eq!(Percentage::new(50.0).to_dmx(&parameter), Dmx::new(128));
        assert_eq!(Percentage::new(100.0).to_dmx(&parameter), Dmx::new(255));
    }
}
