extern crate computation;

use computation::smallstep::Node;
use computation::smallstep::environment::*;
use computation::smallstep::machine::*;

fn print_add_multiply_example() {
    Machine::new(
        Node::add(
            Node::multiply(Node::number(5), Node::number(10)),
            Node::multiply(Node::number(3), Node::number(4))
        ),
        Environment::new()
    ).run();
}

fn print_environment_variables_example() {
    let mut env = Environment::new();
    env.add("x".to_string(), Node::number(3));
    env.add("y".to_string(), Node::number(4));
    Machine::new(
        Node::add(Node::variable("x".to_string()), Node::variable("y".to_string())),
        env
    ).run();
}

fn print_assignment_with_variable_example() {
    let statement = Node::assign(
        "x".to_string(),
        Node::add(Node::variable("x".to_string()), Node::number(1))
    );
    let mut statement_env = Environment::new();
    statement_env.add("x".to_string(), Node::number(2));
    let mut machine = Machine::new(statement, statement_env);
    machine.run();
    println!("x: {}", machine.environment.get("x".to_string()));
}

fn print_sequence_example() {
    Machine::new_with_empty_env(
        Node::sequence(
            Node::assign("x".to_string(), Node::add(Node::number(1), Node::number(1))),
            Node::assign("y".to_string(), Node::add(Node::variable("x".to_string()), Node::number(3)))
        )
    ).run();
}

fn print_while_loop_example() {
    let mut env = Environment::new();
    env.add("x".to_string(), Node::number(1));
    let node = Node::while_node(
        Node::less_than(Node::variable("x".to_string()), Node::number(4)),
        Node::assign("x".to_string(), Node::add(Node::variable("x".to_string()), Node::number(1)))
    );
    Machine::new(node, env).run();
}

fn main() {
    println!("---");
    print_add_multiply_example();

    println!("---");
    print_environment_variables_example();

    println!("---");
    print_assignment_with_variable_example();

    println!("---");
    print_sequence_example();

    println!("---");
    print_while_loop_example();
}
