use smallstep::Node;
use smallstep::environment::Environment;

pub struct Machine {
    expression: Box<Node>,
    pub environment: Environment
}

impl Machine {
    pub fn new(expression: Box<Node>, environment: Environment) -> Machine {
        Machine{ expression: expression, environment: environment }
    }

    pub fn new_with_empty_env(expression: Box<Node>) -> Machine {
        Machine{ expression: expression, environment: Environment::new() }
    }

    pub fn step(&mut self) {
        self.expression = self.expression.reduce(&mut self.environment);
    }

    pub fn run(&mut self) {
        while self.expression.reducable() {
            println!("{}", self.expression);
            self.step();
        }
        println!("{}", self.expression);
    }
}

#[test]
fn test_create_new_machine() {
    let machine = Machine::new(Node::number(1), Environment::new());
    assert_eq!(1, machine.expression.value());
}

#[test]
fn test_create_new_machine_with_empty_environment() {
    let machine = Machine::new_with_empty_env(Node::number(2));
    assert_eq!(2, machine.expression.value());
}

#[test]
fn test_run_more_complex_ast() {
    let add = Node::add(
        Node::multiply(Node::number(2), Node::number(4)),
        Node::multiply(Node::number(3), Node::number(5)),
    );
    let mut machine = Machine::new_with_empty_env(add);
    machine.run();
    assert_eq!(23, machine.expression.value());
}

#[test]
fn test_run_complex_ast_with_variables() {
    let mut env = Environment::new();
    env.add("y".to_string(), Node::number(2));
    env.add("x".to_string(), Node::add(Node::variable("y".to_string()), Node::number(10)));

    let mult = Node::multiply(
        Node::add(Node::number(3), Node::variable("x".to_string())),
        Node::number(10)
    );
    let mut machine = Machine::new(mult, env);
    // (3 + (2 + 10)) * 10
    machine.run();
    assert_eq!(150, machine.expression.value());
}

#[test]
fn test_environment_after_assignment() {
    let assign = Node::assign("x".to_string(), Node::number(5));

    let mut machine = Machine::new_with_empty_env(assign);
    machine.run();

    assert_eq!(5, machine.environment.get("x".to_string()).value());
}
