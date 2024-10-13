use digital_circuit::gates::{Logic, AND, OR, NOT, NAND, NOR, XOR, XNOR};

fn main() {
    let and = AND { a: true, b: true }.logic();
    let or = OR { a: false, b: false }.logic();
    let not = NOT { a: false }.logic();
    let nand = NAND { a: false, b: false }.logic();
    let nor = NOR { a: false, b: true }.logic();
    let xor = XOR { a: true, b: true }.logic();
    let xnor = XNOR { a: true, b: true }.logic();

    println!("AND {}", and);
    println!("OR {}", or);
    println!("NOT {}", not);
    println!("NAND {}", nand);
    println!("NOR {}", nor);
    println!("XOR {}", xor);
    println!("XNOR {}", xnor);
}
