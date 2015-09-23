use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use std::collections::HashMap;

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
        match self.vars.get(&name) {
            Some(v) => { v.clone() }
            None => panic!("Variable {} not found", name)
        }
    }

    pub fn insert(&mut self, name: String, node: Box<Node>) {
        self.vars.insert(name, node);
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut parts = Vec::new();
        for (variable, value) in self.vars.iter() {
            parts.push(format!("{0}={1}", variable, value))
        };
        let text = parts.connect(", ");
        write!(f, "({0})", text)
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

#[test]
fn test_write_environment_with_variables() {
    let mut env = Environment::new();
    env.add("y".to_string(), Node::number(4));
    env.add("x".to_string(), Node::number(5));
    // assert_eq!("(y=4, x=5)".to_string(), env.to_string());
}
