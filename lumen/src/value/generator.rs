use crate::value::convertable::Convertable;
use std::fmt::Debug;
use std::time::Duration;

use crate::parameter::Parameter;
use crate::value::Value;

use super::convertable::{LiteralConverter, PercentageConverter};
use super::Values;

pub trait Generator: Debug {
    fn generate(&self, elapsed: Duration, parameter: &Parameter) -> Values;
}

#[derive(Debug)]
pub struct Static {
    value: Values,
}

impl Static {
    pub fn new(value: Values) -> Self {
        Self { value }
    }
}

impl Generator for Static {
    fn generate(&self, _elapsed: Duration, _parameter: &Parameter) -> Values {
        self.value
    }
}

#[derive(Debug)]
pub struct Fade<G> {
    start: G,
    end: G,
    duration: Duration,
}

impl<G> Fade<G>
where
    G: Generator,
{
    pub fn new(start: G, end: G, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
        }
    }
}

impl<G> Fade<G>
where
    G: Generator,
{
    fn fade_between<V: Value>(&self, start: V, end: V, elapsed: Duration) -> f32 {
        if elapsed > self.duration {
            return end.value();
        }

        let difference = end.value() - start.value();
        let factor = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        let new_value = start.value() + (difference * factor);

        new_value
    }
}

impl<G> Generator for Fade<G>
where
    G: Generator,
{
    fn generate(&self, elapsed: Duration, parameter: &Parameter) -> Values {
        let start = self.start.generate(elapsed, parameter);
        let end = self.end.generate(elapsed, parameter);

        match (start, end) {
            (Values::Literal(start), non_literal_end) => {
                let end = non_literal_end.convert(&LiteralConverter::new(parameter));
                Values::make_literal(self.fade_between(start, end, elapsed))
            }
            (Values::Percentage(start), non_percentage_end) => {
                let end = non_percentage_end.convert(&PercentageConverter::new(parameter));
                Values::make_percentage(self.fade_between(start, end, elapsed))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_always_returns_same_value() {
        let value = Values::make_literal(50.0);
        let static_generator = Static::new(value);
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(
            static_generator.generate(Duration::new(0, 0), &parameter),
            Values::make_literal(50.0),
        );
        assert_eq!(
            static_generator.generate(Duration::new(2, 0), &parameter),
            Values::make_literal(50.0),
        );
        assert_eq!(
            static_generator.generate(Duration::new(4, 0), &parameter),
            Values::make_literal(50.0),
        );
    }

    #[test]
    fn fade_between_like_values() {
        let start = Static::new(Values::make_literal(0.0));
        let end = Static::new(Values::make_literal(100.0));
        let fade = Fade::new(start, end, Duration::new(2, 0));
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(
            fade.generate(Duration::new(0, 0), &parameter),
            Values::make_literal(0.0)
        );
        assert_eq!(
            fade.generate(Duration::new(1, 0), &parameter),
            Values::make_literal(50.0)
        );
        assert_eq!(
            fade.generate(Duration::new(2, 0), &parameter),
            Values::make_literal(100.0)
        );
    }

    #[test]
    fn fade_between_differing_values() {
        let start = Static::new(Values::make_literal(25.0));
        let end = Static::new(Values::make_percentage(100.0));
        let fade = Fade::new(start, end, Duration::new(2, 0));
        let parameter = Parameter::new(0, 25.0, 75.0);

        assert_eq!(
            fade.generate(Duration::new(0, 0), &parameter),
            Values::make_literal(25.0)
        );
        assert_eq!(
            fade.generate(Duration::new(1, 0), &parameter),
            Values::make_literal(50.0)
        );
        assert_eq!(
            fade.generate(Duration::new(2, 0), &parameter),
            Values::make_literal(75.0)
        );
    }
}
