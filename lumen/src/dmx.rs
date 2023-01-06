use std::{fmt::Debug, slice::Iter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct Dmx(u8);

impl Dmx {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_factor(factor: f64) -> Self {
        Self((255.0 * factor).round() as u8)
    }

    pub fn byte(&self) -> u8 {
        self.0
    }
}

pub struct DmxString {
    string: Vec<Dmx>,
}

impl DmxString {
    pub fn new(size: usize) -> Self {
        Self {
            string: vec![Dmx::new(0); size],
        }
    }

    pub fn set(&mut self, offset: usize, dmx: Dmx) {
        self.string[offset] = dmx
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<Dmx> {
        self.string.iter()
    }
}

impl Debug for DmxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for dmx in self.string.iter() {
            write!(f, "{} ", dmx.0).unwrap();
        }

        write!(f, "")
    }
}
