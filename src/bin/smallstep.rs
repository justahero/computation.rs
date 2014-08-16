#![feature(globs)]

extern crate computation;

use computation::smallstep;
use computation::smallstep::Node;
use computation::smallstep::environment::*;
use computation::smallstep::machine::*;

fn main() {
    println!("Number: {}", Node::number(100));
    println!("Boolean: {}", Node::boolean(false));

    let add = Node::add(Node::number(1), Node::number(4));
    println!("Addition: {0}", add);

    let mult = Node::multiply(Node::number(4), Node::number(3));
    println!("Multiplication: {0}", mult);

    let mut test_env = Environment::new();
    test_env.add("x".to_string(), Node::number(2));
    let variable = Node::variable("x".to_string());
    println!("Variable x = {}", variable.reduce(&mut test_env));

    println!("---")
    let mut env = Environment::new();
    env.add("x".to_string(), Node::number(3));
    env.add("y".to_string(), Node::number(4));
    Machine::new(
        Node::add(Node::variable("x".to_string()), Node::variable("y".to_string())),
        env
    ).run();

    println!("---")
    Machine::new(
        Node::add(
            Node::multiply(Node::number(5), Node::number(10)),
            Node::multiply(Node::number(3), Node::number(4))
        ),
        Environment::new()
    ).run();

    println!("---")
    Machine::new_with_empty_env(
        Node::less_than(
            Node::number(10),
            Node::add(Node::number(4), Node::number(5))
        )
    ).run();
}
