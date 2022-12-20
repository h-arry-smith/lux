use crate::timecode::time::Time;
use crate::value::convertable::Convertable;
use std::fmt::Debug;
use std::time::Duration;

use crate::parameter::Parameter;
use crate::value::Value;

use super::convertable::{LiteralConverter, PercentageConverter};
use super::Values;

pub trait Generator: Debug + GeneratorClone {
    fn generate(&mut self, time: &Time, parameter: &Parameter) -> Values;
}

pub trait GeneratorClone {
    fn clone_box(&self) -> Box<dyn Generator>;
}

impl<G> GeneratorClone for G
where
    G: 'static + Generator + Clone,
{
    fn clone_box(&self) -> Box<dyn Generator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Generator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct Static {
    value: Values,
}

impl Static {
    pub fn new(value: Values) -> Self {
        Self { value }
    }
}

impl Generator for Static {
    fn generate(&mut self, _time: &Time, _parameter: &Parameter) -> Values {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct Fade<G> {
    start: G,
    end: G,
    duration: Duration,
    start_time: Option<Duration>,
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
            start_time: None,
        }
    }
}

impl<G> Fade<G>
where
    G: Generator,
{
    fn fade_between<V: Value>(&mut self, start: V, end: V, elapsed: Duration) -> f32 {
        let fade_elapsed_time = self.fade_relative_elapsed_time(elapsed);

        if fade_elapsed_time > self.duration {
            return end.value();
        }

        // The first time the fade is called, we store the time as a start reference to
        // the elapsed global time, and use this to calculate how far we are in the
        // fade
        if self.start_time.is_none() {
            self.start_time = Some(elapsed);
        }

        let difference = end.value() - start.value();
        let factor = fade_elapsed_time.as_secs_f32() / self.duration.as_secs_f32();

        start.value() + (difference * factor)
    }

    fn fade_relative_elapsed_time(&self, elapsed: Duration) -> Duration {
        match self.start_time {
            Some(start) => elapsed.checked_sub(start).unwrap_or_default(),
            None => Duration::new(0, 0),
        }
    }
}

impl<G> Generator for Fade<G>
where
    G: Generator + Clone + 'static,
{
    fn generate(&mut self, time: &Time, parameter: &Parameter) -> Values {
        let start = self.start.generate(time, parameter);
        let end = self.end.generate(time, parameter);

        let elapsed: Duration = (*time).into();

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
    use crate::time;

    #[test]
    fn static_always_returns_same_value() {
        let value = Values::make_literal(50.0);
        let mut static_generator = Static::new(value);
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(
            static_generator.generate(&time!(0 0 0 0 Thirty), &parameter),
            Values::make_literal(50.0),
        );
        assert_eq!(
            static_generator.generate(&time!(0 0 2 0 Thirty), &parameter),
            Values::make_literal(50.0),
        );
        assert_eq!(
            static_generator.generate(&time!(0 0 4 0 Thirty), &parameter),
            Values::make_literal(50.0),
        );
    }

    #[test]
    fn fade_between_like_values() {
        let start = Static::new(Values::make_literal(0.0));
        let end = Static::new(Values::make_literal(100.0));
        let mut fade = Fade::new(start, end, Duration::new(2, 0));
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(
            fade.generate(&time!(0 0 0 0 Thirty), &parameter),
            Values::make_literal(0.0)
        );
        assert_eq!(
            fade.generate(&time!(0 0 1 0 Thirty), &parameter),
            Values::make_literal(50.0)
        );
        assert_eq!(
            fade.generate(&time!(0 0 2 0 Thirty), &parameter),
            Values::make_literal(100.0)
        );
    }

    #[test]
    fn fade_between_differing_values() {
        let start = Static::new(Values::make_literal(25.0));
        let end = Static::new(Values::make_percentage(100.0));
        let mut fade = Fade::new(start, end, Duration::new(2, 0));
        let parameter = Parameter::new(0, 25.0, 75.0);

        assert_eq!(
            fade.generate(&time!(0 0 0 0 Thirty), &parameter),
            Values::make_literal(25.0)
        );
        assert_eq!(
            fade.generate(&time!(0 0 1 0 Thirty), &parameter),
            Values::make_literal(50.0)
        );
        assert_eq!(
            fade.generate(&time!(0 0 2 0 Thirty), &parameter),
            Values::make_literal(75.0)
        );
    }
}
