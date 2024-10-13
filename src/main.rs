use digital_circuit::logic::{Logic, AND};

fn main() {
    let and = AND { a: true, b: false }.logic();

    println!(" {}", and);
}
