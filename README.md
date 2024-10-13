# Digital Logic Circuit

Implementation of digital logic circuit in Rust. [Digital Logic](https://github.com/EteimZ/digital_logic.git) The goal of this repo is to implement digital logic circuits in Rust that will serve as foundation of any other project that seeks to use the these circuit. 

## Gates

To use the logic gate
```
 use digital_circuit::logic::{Logic, AND};

 let and = AND { a: 1, b: 7 }.logic();

 match and {
        Ok(result) => println!(" {}", result),
        Err(e) => println!("Error: {}", e),
    }
 ```
 
