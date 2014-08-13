mod smallstep;

fn main() {
    use smallstep::Number;
    use smallstep::Boolean;
    use smallstep::Add;
    use smallstep::machine::Machine;
    use smallstep::Multiply;

    let number = Number::new(100);
    let attrib = Boolean::new(false);
    let add  = Add::new(Number::new(1), Number::new(4));
    let mult = Multiply::new(Number::new(4), Number::new(3));
    let comb = Add::new(
        Multiply::new(Number::new(5), Number::new(10)),
        Multiply::new(Number::new(3), Number::new(4)),
    );

    let machine = Machine::new(comb.clone());

    println!("Number: {}", number);
    println!("Number reducable: {}", number.reducable());
    println!("Boolean: {}", attrib);
    println!("Boolean reducable: {}", attrib.reducable());
    println!("Addition: {0}", add);
    println!("Addition reducable: {}", add.reducable());
    println!("Multiplication: {0}", mult);
    println!("Multiplication reducable {0}", mult.reducable());

    println!("Running Machine")
    machine.run();
}
