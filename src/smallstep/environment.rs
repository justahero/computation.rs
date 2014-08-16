use std::collections::hashmap::HashMap;

use smallstep::Node;

pub struct Environment {
    pub vars: HashMap<String, Box<Node>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{ vars: HashMap::new() }
    }

    pub fn add(&mut self, name: String, node: Box<Node>) {
        self.vars.insert(name.to_string(), node);
    }

    pub fn get(&self, name: String) -> Box<Node> {
        match self.vars.find(&(name)) {
            Some(v) => { v.clone() }
            None => fail!("Variable {} not found", name)
        }
    }

    pub fn insert(&mut self, name: String, node: Box<Node>) {
        self.vars.insert(name, node);
    }
}

#[test]
fn test_add_variable_to_environment() {
    let mut env = Environment::new();
    env.add("x".to_string(), Node::number(1));
    assert_eq!(1, env.get("x".to_string()).value());
    assert_eq!("1".to_string(), env.get("x".to_string()).to_string());
}

#[test]
fn test_insert_variable_to_environment() {
    let mut env = Environment::new();
    env.add("x".to_string(), Node::number(2));
    env.insert("x".to_string(), Node::number(4));
    assert_eq!(4, env.get("x".to_string()).value());
}
