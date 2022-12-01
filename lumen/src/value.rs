#[derive(Clone, Copy, Debug)]
pub struct Literal {
    pub value: i32,
}

impl Literal {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
