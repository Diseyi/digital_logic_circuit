# Digital Logic Circuit

Implementation of digital logic circuit in Rust. [Digital Logic](https://github.com/EteimZ/digital_logic.git) The goal of this repo is to implement digital logic circuits in Rust that will serve as foundation of any other project that seeks to use the these circuit. 

## Gates

The logic gates takes in two inputs and returns an output

To use the logic gate

``` Rust
 use digital_circuit::logic::{Logic, AND};

    let and = AND { a: true, b: true }.logic();
    let or = OR { a: false, b: false }.logic();
    let not = NOT { a: false }.logic();
    let nand = NAND { a: false, b: false }.logic();
    let nor = NOR { a: false, b: true }.logic();
    let xor = XOR { a: true, b: true }.logic();
    let xnor = XNOR { a: true, b: true }.logic();

    println!("AND {}", and); // true
    println!("OR {}", or); // false
    println!("NOT {}", not); // true
    println!("NAND {}", nand); // true
    println!("NOR {}", nor); // false
    println!("XOR {}", xor); // false
    println!("XNOR {}", xnor); // true
 ```
 
