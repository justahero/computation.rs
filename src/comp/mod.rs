use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum Node {
    Number(int),
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>)
}

impl Node {
    pub fn reducable(&self) -> bool {
        match *self {
            Number(_) => { false }
            _ => { true }
        }
    }
}

impl Show for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Number(value)          => write!(f, "{}", value),
            Add(ref l, ref r)      => write!(f, "{0} + {1}", l, r),
            Multiply(ref l, ref r) => write!(f, "{0} * {1}", l, r)
        }
    }
}

pub struct Number {
    pub value: int
}

impl Number {
    pub fn new(value: int) -> Node {
        Number(value)
    }
}

pub struct Add {
    pub left: Node,
    pub right: Node
}

impl Add {
    pub fn new(left: Node, right: Node) -> Node {
        Add(box left, box right)
    }
}

pub struct Multiply {
    pub left: Node,
    pub right: Node,
}

impl Multiply {
    pub fn new(left: Node, right: Node) -> Node {
        Multiply(box left, box right)
    }
}
