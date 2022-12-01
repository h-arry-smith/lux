#[derive(Clone, Copy, Debug)]
pub struct Literal {
    pub value: f32,
}

impl Literal {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
