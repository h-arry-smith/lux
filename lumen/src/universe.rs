use std::collections::HashMap;

use crate::{
    address::Address,
    dmx::{Dmx, DmxString},
};

#[derive(Debug)]
pub struct Multiverse {
    universes: HashMap<usize, Universe>,
}

impl Multiverse {
    pub fn new() -> Self {
        Self {
            universes: HashMap::new(),
        }
    }

    pub fn map_string(&mut self, address: &Address, dmx_string: &DmxString) {
        match self.universes.get_mut(&address.universe_index()) {
            Some(universe) => universe.map_string(address, dmx_string),
            None => {
                // If we don't have that universe allocated, let's make it.
                let mut universe = Universe::new(address.universe_index());
                universe.map_string(address, dmx_string);
                self.universes.insert(address.universe_index(), universe);
            }
        }
    }

    pub fn universes(&self) -> impl Iterator<Item = &Universe> {
        self.universes.values()
    }
}

impl Default for Multiverse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Universe {
    index: usize,
    values: [Dmx; 512],
}

impl Universe {
    fn new(index: usize) -> Self {
        Self {
            index,
            values: [Dmx::new(0); 512],
        }
    }

    fn map_string(&mut self, address: &Address, dmx_string: &DmxString) {
        let start = address.address_index();

        // TODO: If the dmx string doesn't fit we should really propogate some
        //       error.
        if (start + dmx_string.len()) > self.values.len() {
            return;
        }

        for (i, value) in dmx_string.iter().enumerate() {
            self.values[start + i] = *value;
        }
    }

    pub fn bytes(&self) -> [u8; 512] {
        self.values.map(|dmx| dmx.byte())
    }

    pub fn universe_number(&self) -> usize {
        self.index + 1
    }
}
