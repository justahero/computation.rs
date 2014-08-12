use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

pub trait Reducable {
    fn reducable(&self) -> bool;
}

pub struct Number {
    pub value: int
}

impl Number {
    pub fn new(value: int) -> Number {
        Number{ value: value }
    }
}

// implement Show trait for Number
impl Show for Number {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Reducable for Number {
    fn reducable(&self) -> bool {
        false
    }
}

pub struct Add {
    pub left: Reducable,
    pub right: Reducable
}

impl Add {
    pub fn new(left: Reducable, right: Reducable) -> Add {
        Add{ left: left, right: right }
    }
}

impl Show for Add {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{0} + {1}", self.left, self.right)
    }
}

impl Reducable for Add {
    fn reducable(&self) -> bool { true }
}

pub struct Multiply {
    pub left: Reducable,
    pub right: Reducable
}

impl Multiply {
    pub fn new(left: Reducable, right: Reducable) -> Multiply {
        Multiply{ left: left, right: right }
    }
}

impl Show for Multiply {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{0} * {1}", self.left, self.right)
    }
}

impl Reducable for Multiply {
    fn reducable(&self) -> bool { true }
}
