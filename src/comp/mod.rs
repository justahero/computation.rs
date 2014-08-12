use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

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

pub struct Add {
    pub left: Number,
    pub right: Number
}

impl Add {
    pub fn new(left: Number, right: Number) -> Add {
        Add{ left: left, right: right }
    }
}

impl Show for Add {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{0} + {1}", self.left, self.right)
    }
}

pub struct Multiply {
    pub left: Number,
    pub right: Number
}

impl Multiply {
    pub fn new(left: Number, right: Number) -> Multiply {
        Multiply{ left: left, right: right }
    }
}

impl Show for Multiply {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{0} * {1}", self.left, self.right)
    }
}
