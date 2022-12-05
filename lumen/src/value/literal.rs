use super::Value;
use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq)]
pub struct Literal {
    pub value: f32,
}

impl Literal {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Value for Literal {
    fn value(&self) -> f32 {
        self.value
    }

    fn set(&mut self, value: f32) {
        self.value = value
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.02}", self.value)
    }
}
