# [RPN-Reckoner](https://crates.io/crates/rpn-reckoner/)

DATE: Jul. 7th, 2026

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
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 TOML                      1           18           16            0            2
─────────────────────────────────────────────────────────────────────────────────
 Markdown                  1           82            0           59           23
 |- BASH                   1            2            2            0            0
 |- Rust                   1           16           12            0            4
 (Total)                              100           14           59           27
─────────────────────────────────────────────────────────────────────────────────
 Rust                      4          176          133            9           34
 |- Markdown               1           27            0           27            0
 (Total)                              203          133           36           34
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                     6          321          163           95           63
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Reference

- [『高効率言語 Rust 書きかた・作りかた』](https://www.socym.co.jp/book/1351)

![ref1](https://www.socym.co.jp/wp-content/uploads/2022/120Px_syoei-1.jpg)

## License

- [MIT](https://github.com/Rei-Ashine/rpn-reckoner/blob/main/LICENSE)
