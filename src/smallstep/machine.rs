use smallstep::Node;

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
