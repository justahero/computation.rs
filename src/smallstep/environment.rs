use std::collections::hashmap::HashMap;

use smallstep::Node;

pub struct Environment {
    pub vars: HashMap<String, Box<Node>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{ vars: HashMap::new() }
    }
}