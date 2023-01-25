use std::collections::HashMap;

use lazy_static::lazy_static;

use lumen::parameter::Param;

// TODO: Implement a lazy static for this

lazy_static! {
    pub static ref GROUP_PARAMETERS: HashMap<&'static str, Vec<Param>> = {
        let mut hashmap = HashMap::new();
        hashmap.insert("position", vec![Param::Pan, Param::Tilt]);
        hashmap
    };
}
