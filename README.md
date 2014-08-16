computation.rs
==============

A few samples from the Understanding Computation book written in Rust

SmallSteps
----------

Small steps semantics is used to create an AST that can be reduced step by step until no reduction is possible and evaluates to a final value.

To create a simple number.

```rust
let number = Node::number(2);
// => 2
```

There is a Machine type that can reduce an existing AST with an Environment object and prints out the reduction steps.

```rust
Machine::new_with_empty_env(
    Node::add(Node::number(4), Node::number(10)),
).run();
// => 4 + 10
// => 14
```

It is also possible to use comparison and evaluate to bool values.

```rust
Machine::new_with_empty_env(
    Node::less_than(
        Node::number(10),
        Node::add(Node::number(4), Node::number(5))
    ),
    Environment::new()
).run();
// => 10 < 4 + 5
// => 10 < 9
// => false
```

The small steps lib supports variables that can be make known to the Environment and assignments for generating entries in the Environment object.

```rust
let statement = Node::assign(
    "x".to_string(),
    Node::add(Node::variable("y".to_string()), Node::number(5))
);
let mut env = Environment::new();
env.add("y".to_string(), Node::multiply(Node::number(2), Node::number(6));
let mut machine = Machine::new(statement, env);
machine.run();
// => x = y + 5,  (y = 2 * 6)
// => x = y + 5,  (y = 12)
// => x = 12 + 5, (y = 12)
// => x = 17,     (y = 12)
// => do-nothing, (x = 17, y = 12)
```

Conditionals can be expressed with If nodes.

```rust
let statement = Node::if_else_cond(Node::boolean(true), Node::number(1), Node::number(10));
Machine::new(statement, Environment::new()).run();
// => if true 1 else 10
// => 1
```
