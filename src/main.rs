use digital_circuit::logic::{Logic, AND, OR, NOT};

fn main() {
    let and = AND { a: true, b: true }.logic();
    let or = OR { a: false, b: false }.logic();
    let not = NOT { a: false }.logic();

    println!(" {}", and);
    println!(" {}", or);
    println!(" {}", not);
}
