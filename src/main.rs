use digital_circuit::logic::{Logic, AND};

fn main() {
    let and = AND { a: 1, b: 7 }.logic();

    match and {
        Ok(result) => println!(" {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
