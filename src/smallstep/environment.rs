use std::collections::hashmap::HashMap;

use smallstep::Node;

pub struct Environment {
    pub vars: HashMap<String, Box<Node>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{ vars: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, node: Box<Node>) {
        self.vars.insert(name.to_string(), node);
    }

    pub fn get(&self, name: String) -> Box<Node> {
        match self.vars.find(&(name)) {
            Some(v) => { v.clone() }
            None => fail!("Variable {} not found", name)
        }
    }
}