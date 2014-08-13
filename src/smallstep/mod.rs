use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

#[deriving(Clone)]
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
    pub fn value(&self) -> int {
        match *self {
            Number(v) => { v },
            _ => fail!("Type has no value: {}", *self)
        }
    }
    pub fn reduce(&self) -> Node {
        match *self {
            Add(ref l, ref r) => {
                if l.reducable() {
                    Add(box l.reduce(), r.clone())
                } else if r.reducable() {
                    Add(l.clone(), box r.reduce())
                } else {
                    Number(l.value() + r.value())
                }
            }
            Multiply(ref l, ref r) => {
                if l.reducable() {
                    Multiply(box l.reduce(), r.clone())
                } else if r.reducable() {
                    Multiply(l.clone(), box r.reduce())
                } else {
                    Number(l.value() * r.value())
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

pub struct Machine {
    pub expression: Node
}

impl Machine {
    pub fn new(expression: Node) -> Machine {
        Machine{ expression: expression }
    }

    pub fn run(&self) {
        let mut node = self.expression.clone();
        while node.reducable() {
            println!("{}", node);
            node = node.reduce();
        }
        println!("{}", node);
    }
}