use std::collections::HashMap;

use lumen::parameter::Param;

// TODO: Implement a lazy static for this

pub struct GroupParameters {
    parameters: HashMap<Param, Vec<Param>>,
}

impl GroupParameters {
    pub fn new(parameters: HashMap<Param, Vec<Param>>) -> Self {
        Self { parameters }
    }

    pub fn get(&self, param: &Param) -> Option<&Vec<Param>> {
        self.parameters.get(param)
    }
}

impl Default for GroupParameters {
    fn default() -> Self {
        let mut parameters = HashMap::new();

        parameters.insert(Param::Position, vec![Param::Pan, Param::Tilt]);

        Self::new(parameters)
    }
}
