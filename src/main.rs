mod comp;

fn main() {
    use comp::Number;
    use comp::Add;
    use comp::Multiply;

    let number = Number::new(100);
    let add  = Add::new(Number::new(1), Number::new(4));
    let mult = Multiply::new(Number::new(4), Number::new(3));
    let comb = Add::new(
        Multiply::new(Number::new(5), Number::new(10)),
        Multiply::new(Number::new(3), Number::new(4)),
    );

    println!("Addition: {0}", add);
    println!("Multiplication: {0}", mult);
    println!("Number reducable: {}", Number::new(3).reducable);
    println!("Combination: {0}", comb);
}
