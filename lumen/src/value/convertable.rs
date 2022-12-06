use super::*;

pub trait Converter {
    type Result;
    fn convert_literal(&self, literal: &Literal) -> Self::Result;
    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result;
}

pub trait Convertable<T> {
    fn convert(&self, converter: &dyn Converter<Result = T>) -> T;
}

pub struct LiteralConverter {}
impl Converter for LiteralConverter {
    type Result = Literal;

    fn convert_literal(&self, literal: &Literal) -> Self::Result {
        *literal
    }

    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result {
        // TODO: Converting from a percentage to a literal requires context of
        //       the fixture profile, which requires all the patching and profile
        //       machinery to be in place.
        //
        //       For now, we just return a straight conversion
        Literal::new(percentage.value())
    }
}

pub struct PercentageConverter {}
impl Converter for PercentageConverter {
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
