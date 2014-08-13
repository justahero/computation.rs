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
}

impl Node {
    pub fn number(value: int) -> Box<Node> { box Number(value) }

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
                    Add::new(l.reduce(environment), r.clone())
                } else if r.reducable() {
                    Add::new(l.clone(), r.reduce(environment))
                } else {
                    Node::number(l.value() + r.value())
                }
            }
            Multiply(ref l, ref r) => {
                if l.reducable() {
                    Multiply::new(l.reduce(environment), r.clone())
                } else if r.reducable() {
                    Multiply::new(l.clone(), r.reduce(environment))
                } else {
                    Node::number(l.value() * r.value())
                }
            }
            LessThan(ref l, ref r) => {
                if l.reducable() {
                    LessThan::new(l.reduce(environment), r.clone())
                } else if r.reducable() {
                    LessThan::new(l.clone(), r.reduce(environment))
                } else {
                    Boolean::new(l.value() < r.value())
                }
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
        }
    }
}

pub struct Add {
    pub left: Box<Node>,
    pub right: Box<Node>
}

impl Add {
    pub fn new(left: Box<Node>, right: Box<Node>) -> Box<Node> {
        box Add(left, right)
    }
}

pub struct Multiply {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Multiply {
    pub fn new(left: Box<Node>, right: Box<Node>) -> Box<Node> {
        box Multiply(left, right)
    }
}

pub struct Boolean {
    pub value: bool
}

impl Boolean {
    pub fn new(value: bool) -> Box<Node> {
        box Boolean(value)
    }
}

pub struct LessThan {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl LessThan {
    pub fn new(left: Box<Node>, right: Box<Node>) -> Box<Node> {
        box LessThan(left, right)
    }
}
