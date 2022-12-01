use std::time::Duration;

use crate::value::Literal;

#[derive(Debug)]
pub struct StaticGenerator {
    value: Literal,
}

impl StaticGenerator {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }

    pub fn generate(&self, _elapsed: Duration) -> Literal {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_returns_same_value() {
        let value = Literal::new(50);
        let static_generator = StaticGenerator::new(value);

        assert_eq!(static_generator.generate(Duration::new(0, 0)).value, 50);
        assert_eq!(static_generator.generate(Duration::new(2, 0)).value, 50);
        assert_eq!(static_generator.generate(Duration::new(4, 0)).value, 50);
    }
}
