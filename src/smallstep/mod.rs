use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

use smallstep::environment::Environment;

pub mod machine;
pub mod environment;

#[deriving(Clone)]
pub enum Node {
    Number(int),
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Boolean(bool),
    LessThan(Box<Node>, Box<Node>),
    Variable(String),
}

impl Node {
    pub fn number(value: int) -> Box<Node> { box Number(value) }

    pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Node> { box Add(left, right) }

    pub fn multiply(left: Box<Node>, right: Box<Node>) -> Box<Node> { box Multiply(left, right) }

    pub fn boolean(value: bool) -> Box<Node> { box Boolean(value) }

    pub fn less_than(left: Box<Node>, right: Box<Node>) -> Box<Node> { box LessThan(left, right) }

    pub fn variable(name: &str) -> Box<Node> { box Variable(name.to_string()) }

    pub fn reducable(&self) -> bool {
        match *self {
            Number(_)  => { false }
            Boolean(_) => { false }
            _ => { true }
        }
    }

    pub fn condition(&self) -> bool {
        match *self {
            Boolean(b) => { b }
            _ => fail!("Type has no value: {}", *self)
        }
    }

    pub fn value(&self) -> int {
        match *self {
            Number(v)  => { v }
            _ => fail!("Type has no value: {}", *self)
        }
    }
    pub fn reduce(&self, environment: &mut Environment) -> Box<Node> {
        match *self {
            Add(ref l, ref r) => {
                if l.reducable() {
                    Node::add(l.reduce(environment), r.clone())
                } else if r.reducable() {
                    Node::add(l.clone(), r.reduce(environment))
                } else {
                    Node::number(l.value() + r.value())
                }
            }
            Multiply(ref l, ref r) => {
                if l.reducable() {
                    Node::multiply(l.reduce(environment), r.clone())
                } else if r.reducable() {
                    Node::multiply(l.clone(), r.reduce(environment))
                } else {
                    Node::number(l.value() * r.value())
                }
            }
            LessThan(ref l, ref r) => {
                if l.reducable() {
                    Node::less_than(l.reduce(environment), r.clone())
                } else if r.reducable() {
                    Node::less_than(l.clone(), r.reduce(environment))
                } else {
                    Node::boolean(l.value() < r.value())
                }
            }
            Variable(ref name) => {
                environment.get(name.clone())
            }
            _ => fail!("Non reducable type found: {}", *self)
        }
    }
}

impl Show for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Number(value)          => write!(f, "{}", value),
            Add(ref l, ref r)      => write!(f, "{0} + {1}", l, r),
            Multiply(ref l, ref r) => write!(f, "{0} * {1}", l, r),
            Boolean(value)         => write!(f, "{}", value),
            LessThan(ref l, ref r) => write!(f, "{0} < {1}", l, r),
            Variable(ref value)    => write!(f, "{}", value),
        }
    }
}

#[test]
fn test_creates_number() {
    let number = Node::number(2);
    assert_eq!(false, number.reducable());
    assert_eq!(2, number.value());
    assert_eq!("2".to_string(), number.to_string());
}

#[test]
fn test_creates_boolean() {
    let val = Node::boolean(true);
    assert_eq!(false, val.reducable());
    assert_eq!(true, val.condition());
    assert_eq!("true".to_string(), val.to_string());
}

#[test]
fn test_creates_add_node() {
    let add = Node::add(Node::number(4), Node::number(5));
    assert_eq!(true, add.reducable());
    assert_eq!("4 + 5".to_string(), add.to_string());
}

#[test]
fn test_reduce_add_node() {
    let add = Node::add(Node::number(5), Node::number(10));
    let mut env = Environment::new();
    assert_eq!(15, add.reduce(&mut env).value());
    assert_eq!("15".to_string(), add.reduce(&mut env).to_string());
}

#[test]
fn test_creates_mulitply_node() {
    let mult = Node::multiply(Node::number(10), Node::number(3));
    assert_eq!(true, mult.reducable());
    assert_eq!("10 * 3".to_string(), mult.to_string());
}

#[test]
fn test_reduce_multiply_node() {
    let mult = Node::multiply(Node::number(5), Node::number(7));
    let mut env = Environment::new();
    assert_eq!(35, mult.reduce(&mut env).value());
    assert_eq!("35".to_string(), mult.reduce(&mut env).to_string());
}

#[test]
fn test_creates_less_than_node() {
    let lessthan = Node::less_than(Node::number(12), Node::number(8));
    assert_eq!(true, lessthan.reducable());
    assert_eq!("12 < 8".to_string(), lessthan.to_string());
}

#[test]
fn test_reduce_less_than_node() {
    let less = Node::less_than(Node::number(7), Node::number(8));
    let mut env = Environment::new();
    assert_eq!(true, less.reduce(&mut env).condition());
    assert_eq!("true".to_string(), less.reduce(&mut env).to_string());
}

#[test]
fn test_create_variable() {
    let var = Node::variable("x");
    assert_eq!("x".to_string(), var.to_string());
}

#[test]
fn test_environment_resolve_variable() {
    let var = Node::variable("y");
    let mut env = Environment::new();
    env.add("y", Node::number(2));
    assert_eq!(2, var.reduce(&mut env).value());
    assert_eq!("2".to_string(), var.reduce(&mut env).to_string());
}
