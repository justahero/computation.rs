use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

use smallstep::environment::Environment;

#[deriving(Clone)]
pub enum Node {
    Number(int),
    Boolean(bool),
    // Add(Box<Node>, Box<Node>),
    // Multiply(Box<Node>, Box<Node>),
    // LessThan(Box<Node>, Box<Node>),
    // Variable(String),
    // DoNothing,
    // Assign(String, Box<Node>),
    // If(Box<Node>, Box<Node>, Box<Node>),
    // Sequence(Box<Node>, Box<Node>),
}

impl Node {
    pub fn number(value: int) -> Box<Node> { Box::new(Number(value)) }
    pub fn boolean(value: bool) -> Box<Node> { Box::new(Boolean(value)) }
    // pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Add(left, right)) }
    // pub fn multiply(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Multiply(left, right)) }
    // pub fn less_than(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(LessThan(left, right)) }
    // pub fn variable(name: String) -> Box<Node> { Box::new(Variable(name)) }
    // pub fn do_nothing() -> Box<Node> { Box::new(DoNothing) }
    // pub fn assign(name: String, expression: Box<Node>) -> Box<Node> { Box::new(Assign(name, expression)) }
    // pub fn if_else_cond(condition: Box<Node>, left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(If(condition, left, right)) }
    // pub fn if_cond(condition: Box<Node>, left: Box<Node>) -> Box<Node> { Box::new(If(condition, left, Node::do_nothing())) }
    // pub fn sequence(first: Box<Node>, second: Box<Node>) -> Box<Node> { Box::new(Sequence(first, second)) }
}

impl Node {
    pub fn value(&self) -> int {
        match *self {
            Number(v)  => { v }
            _ => fail!("Type has no value: {}", *self)
        }
    }

    pub fn condition(&self) -> bool {
        match *self {
            Boolean(b) => { b }
            _ => fail!("Type has no value: {}", *self)
        }
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Box<Node> {
        match *self {
            Number(val) => { Box::new(*self) }
            Boolean(value) => { Box::new(*self) }
        }
    }
}

impl Show for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Number(value)           => write!(f, "{}", value),
            Boolean(value)          => write!(f, "{}", value),
            // Add(ref l, ref r)       => write!(f, "{0} + {1}", l, r),
            // Multiply(ref l, ref r)  => write!(f, "{0} * {1}", l, r),
            // LessThan(ref l, ref r)  => write!(f, "{0} < {1}", l, r),
            // Variable(ref value)     => write!(f, "{}", value),
            // DoNothing               => write!(f, "do-nothing"),
            // Assign(ref n, ref e)    => write!(f, "{0} = {1}", n, e),
            // If(ref c, ref l, ref r) => write!(f, "if ({0}) {1} else {2}", c, l, r),
            // Sequence(ref l, ref r)  => write!(f, "{0}; {1}", l, r),
        }
    }
}

#[test]
fn test_creates_number() {
    let number = Node::number(2);
    assert_eq!(2, number.evaluate(&mut Environment::new()).value());
    assert_eq!("2".to_string(), number.to_string());
}

#[test]
fn test_creates_boolean() {
    let val = Node::boolean(true);
    assert_eq!(true, val.evaluate(&mut Environment::new()).condition());
    assert_eq!("true".to_string(), val.to_string());
}
