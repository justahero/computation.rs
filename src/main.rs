use smallstep::Node;
use smallstep::machine::Machine;
use smallstep::environment::Environment;

mod smallstep;

fn main() {

    println!("Number: {}", Node::number(100));
    println!("Boolean: {}", Node::boolean(false));

    let add = Node::add(Node::number(1), Node::number(4));
    println!("Addition: {0}", add);

    let mult = Node::multiply(Node::number(4), Node::number(3));
    println!("Multiplication: {0}", mult);

    println!("---")
    Machine::new(
        Node::add(
            Node::multiply(Node::number(5), Node::number(10)),
            Node::multiply(Node::number(3), Node::number(4)),
        )
    ).run(&mut Environment::new());

    println!("---")
    Machine::new(
        Node::less_than(Node::number(10), Node::add(Node::number(4), Node::number(5))),
    ).run(&mut Environment::new());
}
