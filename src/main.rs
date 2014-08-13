use smallstep::Node;
use smallstep::Boolean;
use smallstep::LessThan;
use smallstep::Add;
use smallstep::Multiply;
use smallstep::machine::Machine;
use smallstep::environment::Environment;

mod smallstep;

fn main() {

    println!("Number: {}", Node::number(100));
    println!("Boolean: {}", Boolean::new(false));

    let add = Add::new(Node::number(1), Node::number(4));
    println!("Addition: {0}", add);

    let mult = Multiply::new(Node::number(4), Node::number(3));
    println!("Multiplication: {0}", mult);

    println!("---")
    Machine::new(
        Add::new(
            Multiply::new(Node::number(5), Node::number(10)),
            Multiply::new(Node::number(3), Node::number(4)),
        )
    ).run(&mut Environment::new());

    println!("---")
    Machine::new(
        LessThan::new(Node::number(10), Add::new(Node::number(4), Node::number(5))),
    ).run(&mut Environment::new());
}
