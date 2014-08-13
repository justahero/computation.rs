mod smallstep;

fn main() {
    use smallstep::Number;
    use smallstep::Boolean;
    use smallstep::LessThan;
    use smallstep::Add;
    use smallstep::machine::Machine;
    use smallstep::Multiply;

    println!("Number: {}", Number::new(100));
    println!("Boolean: {}", Boolean::new(false));

    let add = Add::new(Number::new(1), Number::new(4));
    println!("Addition: {0}", add);

    let mult = Multiply::new(Number::new(4), Number::new(3));
    println!("Multiplication: {0}", mult);

    let machine = Machine::new(
        Add::new(
            Multiply::new(Number::new(5), Number::new(10)),
            Multiply::new(Number::new(3), Number::new(4)),
        )
    );
    println!("Running Machine")
    machine.run();

    let comparison = Machine::new(
        LessThan::new(Number::new(10), Number::new(20)),
    );
    println!("LessThan")
    comparison.run();
}
