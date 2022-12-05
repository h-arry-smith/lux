use super::Value;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
pub struct Percentage {
    pub percentage: f32,
}

impl Percentage {
    pub fn new(percentage: f32) -> Self {
        Self { percentage }
    }
}

impl Value for Percentage {
    fn value(&self) -> f32 {
        self.percentage
    }

    fn set(&mut self, value: f32) {
        self.percentage = value
    }
}

impl Debug for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.02}%", self.percentage)
    }
}
