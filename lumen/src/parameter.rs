use std::fmt::Display;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Param {
    Intensity,
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
