use std::time::Duration;

use crate::value::Literal;

pub trait Generator {
    fn generate(&self, elapsed: Duration) -> Literal;
}

#[derive(Debug)]
pub struct Static {
    value: Literal,
}

impl Static {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

impl Generator for Static {
    fn generate(&self, _elapsed: Duration) -> Literal {
        self.value
    }
}

#[derive(Debug)]
pub struct Fade {
    start: Literal,
    end: Literal,
    duration: Duration,
}

impl Fade {
    pub fn new(start: Literal, end: Literal, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
        }
    }
}

impl Generator for Fade {
    fn generate(&self, elapsed: Duration) -> Literal {
        if elapsed > self.duration {
            return self.end;
        }

        let difference = self.end.value - self.start.value;
        let factor = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        let new_value = self.start.value + (difference * factor);

        Literal::new(new_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_returns_same_value() {
        let value = Literal::new(50.0);
        let static_generator = Static::new(value);

        assert_eq!(static_generator.generate(Duration::new(0, 0)).value, 50.0);
        assert_eq!(static_generator.generate(Duration::new(2, 0)).value, 50.0);
        assert_eq!(static_generator.generate(Duration::new(4, 0)).value, 50.0);
    }

    #[test]
    fn fade() {
        let start = Literal::new(0.0);
        let end = Literal::new(100.0);
        let fade = Fade::new(start, end, Duration::new(2, 0));

        assert_eq!(fade.generate(Duration::new(0, 0)).value, 0.0);
        assert_eq!(fade.generate(Duration::new(1, 0)).value, 50.0);
        assert_eq!(fade.generate(Duration::new(2, 0)).value, 100.0);
    }
}
