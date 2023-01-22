use crate::timecode::time::Time;
use crate::value::convertable::Convertable;
use std::fmt::{Debug, Display};
use std::time::Duration;

use crate::parameter::Parameter;
use crate::value::Value;

use super::convertable::{LiteralConverter, PercentageConverter};
use super::Values;

pub type BoxedGenerator = Box<dyn Generator + Send + Sync>;

// TODO: This file needs splitting out to multiple other files

pub trait Generator: Debug + GeneratorClone + Display {
    fn generate(&mut self, time: &Time, parameter: &Parameter) -> Option<Values>;
    fn value(&self) -> Values;
    fn set_start_time(&mut self, _time: Time) {}
    fn start_time(&self) -> Time {
        Time::at(0, 0, 0, 0)
    }
    fn resolve(&mut self, value: &Values, time: &Time) {}
}

pub trait GeneratorClone {
    fn clone_box(&self) -> BoxedGenerator;
}

impl<G> GeneratorClone for G
where
    G: 'static + Generator + Clone + Send + Sync,
{
    fn clone_box(&self) -> BoxedGenerator {
        Box::new(self.clone())
    }
}

impl Clone for BoxedGenerator {
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
    fn generate(&mut self, _time: &Time, parameter: &Parameter) -> Option<Values> {
        match self.value {
            Values::Literal(literal) => Some(Values::make_literal(
                literal.value().clamp(parameter.min(), parameter.max()),
            )),
            Values::Percentage(_) => Some(self.value),
        }
    }

    fn value(&self) -> Values {
        self.value
    }
}

impl Display for Static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STATIC({})", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Fade {
    start: BoxedGenerator,
    end: BoxedGenerator,
    duration: Duration,
    start_time: Option<Time>,
}

impl Fade {
    pub fn new(start: BoxedGenerator, end: BoxedGenerator, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
            start_time: None,
        }
    }

    fn fade_between<V: Value>(&mut self, start: V, end: V, elapsed: Duration) -> f64 {
        let fade_elapsed_time = self.fade_relative_elapsed_time(elapsed);

        if fade_elapsed_time > self.duration {
            return end.value();
        }

        let difference = end.value() - start.value();
        let factor = fade_elapsed_time.as_secs_f64() / self.duration.as_secs_f64();

        start.value() + (difference * factor)
    }

    fn fade_relative_elapsed_time(&self, elapsed: Duration) -> Duration {
        match self.start_time {
            Some(start) => elapsed.checked_sub(start.into()).unwrap_or_default(),
            None => Duration::new(0, 0),
        }
    }
}

impl Generator for Fade {
    fn generate(&mut self, time: &Time, parameter: &Parameter) -> Option<Values> {
        let start = self
            .start
            .generate(time, parameter)
            .expect("a fade can not have a delayed value");
        let end = self
            .end
            .generate(time, parameter)
            .expect("a fade can not have a delayed value");

        let elapsed: Duration = (*time).into();

        match (start, end) {
            (Values::Literal(start), non_literal_end) => {
                let end = non_literal_end.convert(&LiteralConverter::new(parameter));
                Some(Values::make_literal(self.fade_between(start, end, elapsed)))
            }
            (Values::Percentage(start), non_percentage_end) => {
                let end = non_percentage_end.convert(&PercentageConverter::new(parameter));
                Some(Values::make_percentage(
                    self.fade_between(start, end, elapsed),
                ))
            }
        }
    }

    // For value inspection of a fade we just return the end value
    fn value(&self) -> Values {
        self.end.value()
    }

    fn set_start_time(&mut self, time: Time) {
        self.start_time = Some(time);
    }

    fn start_time(&self) -> Time {
        self.start_time.unwrap_or_else(|| Time::at(0, 0, 0, 0))
    }

    fn resolve(&mut self, value: &Values, time: &Time) {
        self.start.resolve(value, time);
        self.end.resolve(value, time);
    }
}

impl Display for Fade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FADE({} -> {}, {:.1}s)",
            self.start.value(),
            self.end.value(),
            self.duration.as_secs_f64()
        )
    }
}

#[derive(Debug, Clone)]
pub struct Delay {
    delay: Duration,
    generator: BoxedGenerator,
    start_time: Option<Time>,
}

impl Delay {
    pub fn new(delay: Duration, generator: BoxedGenerator) -> Self {
        Self {
            delay,
            generator,
            start_time: None,
        }
    }

    fn active(&self, time: &Time) -> bool {
        let start_time = match self.start_time {
            Some(time) => time,
            None => Time::at(0, 0, 0, 0),
        };

        let elapsed: Duration = (*time).into();
        let relative_elapsed = elapsed.checked_sub(start_time.into()).unwrap_or_default();

        relative_elapsed >= self.delay
    }
}

impl Generator for Delay {
    fn generate(&mut self, time: &Time, parameter: &Parameter) -> Option<Values> {
        if self.active(time) {
            self.generator.generate(time, parameter)
        } else {
            None
        }
    }

    // For value inspection of a fade we return the delayed generator value
    fn value(&self) -> Values {
        self.generator.value()
    }

    fn set_start_time(&mut self, time: Time) {
        self.start_time = Some(time);

        // The start time of the delay generator is our start time + the delay
        self.generator
            .set_start_time(time + Time::from(&self.delay));
    }

    fn start_time(&self) -> Time {
        // If start time is unset, then assume a start at 0
        self.start_time
            .unwrap_or_else(|| Time::at(0, 0, 0, 0) + Time::from(&self.delay))
    }

    fn resolve(&mut self, value: &Values, time: &Time) {
        if self.active(time) {
            self.generator.resolve(value, time);
        }
    }
}

impl Display for Delay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}s {}", self.delay.as_secs_f64(), self.generator)
    }
}

#[derive(Debug, Clone)]
pub struct CurrentValue {
    generator: Option<BoxedGenerator>,
}

impl CurrentValue {
    pub fn new() -> Self {
        Self { generator: None }
    }
}

impl Generator for CurrentValue {
    fn generate(&mut self, time: &Time, parameter: &Parameter) -> Option<Values> {
        match self.generator {
            Some(ref mut generator) => generator.generate(time, parameter),
            None => Some(Values::make_literal(parameter.min())),
        }
    }

    fn value(&self) -> Values {
        match self.generator {
            Some(ref generator) => generator.value(),
            None => Values::make_literal(0.0),
        }
    }

    fn resolve(&mut self, value: &Values, _time: &Time) {
        if self.generator.is_none() {
            self.generator = Some(Box::new(Static::new(*value)))
        }
    }
}

impl Display for CurrentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CurVal")
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
            Some(Values::make_literal(50.0)),
        );
        assert_eq!(
            static_generator.generate(&time!(0 0 2 0 Thirty), &parameter),
            Some(Values::make_literal(50.0)),
        );
        assert_eq!(
            static_generator.generate(&time!(0 0 4 0 Thirty), &parameter),
            Some(Values::make_literal(50.0)),
        );
    }

    #[test]
    fn fade_between_like_values() {
        let start = Box::new(Static::new(Values::make_literal(0.0)));
        let end = Box::new(Static::new(Values::make_literal(100.0)));
        let mut fade = Fade::new(start, end, Duration::new(2, 0));
        fade.set_start_time(Time::at(0, 0, 0, 0));
        let parameter = Parameter::new(0, 0.0, 100.0);

        assert_eq!(
            fade.generate(&time!(0 0 0 0 Thirty), &parameter),
            Some(Values::make_literal(0.0))
        );
        assert_eq!(
            fade.generate(&time!(0 0 1 0 Thirty), &parameter),
            Some(Values::make_literal(50.0))
        );
        assert_eq!(
            fade.generate(&time!(0 0 2 0 Thirty), &parameter),
            Some(Values::make_literal(100.0))
        );
    }

    #[test]
    fn fade_between_differing_values() {
        let start = Box::new(Static::new(Values::make_literal(25.0)));
        let end = Box::new(Static::new(Values::make_percentage(100.0)));
        let mut fade = Fade::new(start, end, Duration::new(2, 0));
        fade.set_start_time(Time::at(0, 0, 0, 0));
        let parameter = Parameter::new(0, 25.0, 75.0);

        assert_eq!(
            fade.generate(&time!(0 0 0 0 Thirty), &parameter),
            Some(Values::make_literal(25.0))
        );
        assert_eq!(
            fade.generate(&time!(0 0 1 0 Thirty), &parameter),
            Some(Values::make_literal(50.0))
        );
        assert_eq!(
            fade.generate(&time!(0 0 2 0 Thirty), &parameter),
            Some(Values::make_literal(75.0))
        );
    }
}
