use crate::patch::Parameter;

use super::*;

pub trait Converter {
    type Result;
    fn convert_literal(&self, literal: &Literal) -> Self::Result;
    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result;
}

pub trait Convertable<T> {
    fn convert(&self, converter: &dyn Converter<Result = T>) -> T;
}

pub struct LiteralConverter<'a> {
    parameter: &'a Parameter,
}

impl<'a> LiteralConverter<'a> {
    pub fn new(parameter: &'a Parameter) -> Self {
        Self { parameter }
    }
}

impl<'a> Converter for LiteralConverter<'a> {
    type Result = Literal;

    fn convert_literal(&self, literal: &Literal) -> Self::Result {
        *literal
    }

    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result {
        percentage.to_literal(self.parameter)
    }
}

pub struct PercentageConverter<'a> {
    parameter: &'a Parameter,
}

impl<'a> PercentageConverter<'a> {
    pub fn new(parameter: &'a Parameter) -> Self {
        Self { parameter }
    }
}

impl<'a> Converter for PercentageConverter<'a> {
    type Result = Percentage;

    fn convert_literal(&self, literal: &Literal) -> Self::Result {
        // TODO: Converting from a literal to a percentage requires context of
        //       the fixture profile, which requires all the patching and profile
        //       machinery to be in place.
        //
        //       For now, we just return a straight conversion
        Percentage::new(literal.value())
    }

    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result {
        *percentage
    }
}
