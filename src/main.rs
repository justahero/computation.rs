mod comp;

fn main() {
    use comp::Number;


    let number = Number{ value: 2 };
    println!("Number is: {0}", number.value);
}
