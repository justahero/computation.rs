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
    pub fn value(&self) -> int {
        match *self {
            Number(v)  => { v },
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
