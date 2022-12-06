use std::fmt::Debug;
use std::time::Duration;

use crate::value::Value;

use super::Values;

pub trait Generator: Debug {
    fn generate(&self, elapsed: Duration) -> Values;
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
    fn generate(&self, _elapsed: Duration) -> Values {
        self.value
    }
}

#[derive(Debug)]
pub struct Fade {
    start: Values,
    end: Values,
    duration: Duration,
}

impl Fade {
    pub fn new(start: Values, end: Values, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
        }
    }
}

impl Fade {
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

impl Generator for Fade {
    fn generate(&self, elapsed: Duration) -> Values {
        match (self.start, self.end) {
            (Values::Literal(s), Values::Literal(e)) => {
                Values::make_literal(self.fade_between(s, e, elapsed))
            }
            (Values::Percentage(s), Values::Percentage(e)) => {
                Values::make_percentage(self.fade_between(s, e, elapsed))
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_returns_same_value() {
        let value = Values::make_literal(50.0);
        let static_generator = Static::new(value);

        assert_eq!(
            static_generator.generate(Duration::new(0, 0)),
            Values::make_literal(50.0)
        );
        assert_eq!(
            static_generator.generate(Duration::new(2, 0)),
            Values::make_literal(50.0)
        );
        assert_eq!(
            static_generator.generate(Duration::new(4, 0)),
            Values::make_literal(50.0)
        );
    }

    #[test]
    fn fade() {
        let start = Values::make_literal(0.0);
        let end = Values::make_literal(100.0);
        let fade = Fade::new(start, end, Duration::new(2, 0));

        assert_eq!(
            fade.generate(Duration::new(0, 0)),
            Values::make_literal(0.0)
        );
        assert_eq!(
            fade.generate(Duration::new(1, 0)),
            Values::make_literal(50.0)
        );
        assert_eq!(
            fade.generate(Duration::new(2, 0)),
            Values::make_literal(100.0)
        );
    }
}
