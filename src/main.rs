use smallstep::Number;
use smallstep::Boolean;
use smallstep::LessThan;
use smallstep::Add;
use smallstep::Multiply;
use smallstep::machine::Machine;
use smallstep::environment::Environment;

mod smallstep;

fn main() {

    println!("Number: {}", Number::new(100));
    println!("Boolean: {}", Boolean::new(false));

    let add = Add::new(Number::new(1), Number::new(4));
    println!("Addition: {0}", add);

    let mult = Multiply::new(Number::new(4), Number::new(3));
    println!("Multiplication: {0}", mult);

    println!("---")
    Machine::new(
        Add::new(
            Multiply::new(Number::new(5), Number::new(10)),
            Multiply::new(Number::new(3), Number::new(4)),
        )
    ).run(&mut Environment::new());

    println!("---")
    Machine::new(
        LessThan::new(Number::new(10), Add::new(Number::new(4), Number::new(5))),
    ).run(&mut Environment::new());
}
