use std::fmt::Debug;

pub trait Value: Debug {
    fn value(&self) -> f32;
    fn set(&mut self, value: f32);
}

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

#[derive(Clone, Copy)]
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

    fn set(&mut self, value: f32) {}
}

impl Debug for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.02}%", self.percentage)
    }
}
