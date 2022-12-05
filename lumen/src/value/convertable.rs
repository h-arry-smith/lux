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
        todo!()
    }

    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result {
        todo!()
    }
}

pub struct PercentageConverter {}
impl Converter for PercentageConverter {
    type Result = Percentage;

    fn convert_literal(&self, literal: &Literal) -> Self::Result {
        todo!()
    }

    fn convert_percentage(&self, percentage: &Percentage) -> Self::Result {
        todo!()
    }
}
