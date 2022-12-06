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
        match self.universes.get_mut(&address.index()) {
            Some(universe) => universe.map_string(&address, &dmx_string),
            None => {
                // If we don't have that universe allocated, let's make it.
                let mut universe = Universe::new(address.index());
                universe.map_string(&address, &dmx_string);
                self.universes.insert(address.index(), universe);
            }
        }
    }
}

#[derive(Debug)]
struct Universe {
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
        let start = address.address();

        // TODO: If the dmx string doesn't fit we should really propogate some
        //       error.
        if (start + dmx_string.len()) > self.values.len() {
            return;
        }

        for (i, value) in dmx_string.iter().enumerate() {
            self.values[start + i] = *value;
        }
    }
}
