# [RPN-Reckoner](https://crates.io/crates/rpn-reckoner/)

DATE: Feb. 8th, 2023

This package provides a Rust library and  
an executable for doing Reverse Polish Notation (: RPN) calculations.

## Library

This package's library allows for evaluating RPN expressions  
by using the function `rpn_reckoner::eval(expression: &str) -> Result<f64, String>`.  
This function takes an RPN expression as a string argument and returns a result,  
either the evaluated value as a float or an error message as a string.

## Installation

```bash
cargo add rpn_reckoner
```

### Example1

```rust
extern crate rpn_reckoner;


fn main() {
    let expression = String::from("3 2 +");
    let solution = rpn_reckoner::eval(&expression).unwrap();
    println!("{}", solution); // -> 5
}
```

### Example2

```rust
extern crate rpn_reckoner;


fn main() {
    let expression = String::from("6 4 ! +");
    let solution = rpn_reckoner::eval(&expression).unwrap();
    println!("{}", solution); // -> 30
}
```

## Read-Eval-Print Loop (: REPL)

```bash
git clone https://github.com/Rei-Ashine/rpn-reckoner.git
```

To start the REPL for evaluating RPN expressions,  
the command `cargo run` should be run in the terminal.

![REPL](./PNG/RPN-Reckoner_REPL.png)

## Directory Structure

```text
.
├── README.md
├── Cargo.toml
├── LICENSE
└── src
    ├── lib.rs
    ├── main.rs
    ├── operation.rs
    └── test.rs
```

---

```text
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 TOML                    1           20           17            0            3
-------------------------------------------------------------------------------
 Markdown                1           46            0           29           17
 |- Rust                 1           16           12            0            4
 (Total)                             62           12           29           21
-------------------------------------------------------------------------------
 Rust                    4          145          105            8           32
 |- Markdown             1           27            0           27            0
 (Total)                            172          105           35           32
===============================================================================
 Total                   6          211          122           37           52
===============================================================================
```

## Reference

- [『高効率言語 Rust 書きかた・作りかた』](https://www.socym.co.jp/book/1351)

![ref1](https://www.socym.co.jp/wp-content/uploads/2022/120Px_syoei-1.jpg)

## License

- [MIT](https://github.com/Rei-Ashine/rpn-reckoner/blob/main/LICENSE)
