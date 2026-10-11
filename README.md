# [RPN-Reckoner](https://crates.io/crates/rpn-reckoner/)

DATE: Oct. 11th, 2026

This package provides a Rust library and  
an executable for doing Reverse Polish Notation (: RPN) calculations.

## Library

This package's library allows for evaluating RPN expressions  
by using the function `rpn_reckoner::eval(expression: &str) -> Result<f64, String>`.  
This function takes an RPN expression as a string argument and returns a result,  
either the evaluated value as a float or an error message as a string.

## Installation

```bash
cargo add rpn-reckoner
```

This package uses Rust 2024 edition, so Rust 1.85 or later is required.

### Example1

```rust
fn main() {
    let solution = rpn_reckoner::eval("3 2 +").unwrap();
    println!("{}", solution); // -> 5
}
```

### Example2

```rust
fn main() {
    let solution = rpn_reckoner::eval("6 4 ! +").unwrap();
    println!("{}", solution); // -> 30
}
```

## Read-Eval-Print Loop (: REPL)

The REPL can be installed from crates.io and started with the `rpn-reckoner` command.

```bash
cargo install rpn-reckoner
rpn-reckoner
```

Alternatively, clone this repository and run the command `cargo run` in the terminal.

```bash
git clone https://github.com/Rei-Ashine/rpn-reckoner.git
cd rpn-reckoner
cargo run
```

![REPL](./PNG/RPN-Reckoner_REPL.png)

## Directory Structure

```text
.
├── .github
│   └── workflows
│       └── ci.yml
├── README.md
├── Cargo.toml
├── LICENSE
├── PNG
│   ├── RPN-Reckoner_20230208.png
│   └── RPN-Reckoner_REPL.png
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
 TOML                      1           17           16            0            1
─────────────────────────────────────────────────────────────────────────────────
 Markdown                  1           95            0           69           26
 |- BASH                   1            6            6            0            0
 |- Rust                   1            8            8            0            0
 (Total)                              109           14           69           26
─────────────────────────────────────────────────────────────────────────────────
 Rust                      4          219          171            7           41
 |- Markdown               3           38            0           35            3
 (Total)                              257          171           42           44
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                     6          383          201          111           71
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Reference

- [『高効率言語 Rust 書きかた・作りかた』](https://www.socym.co.jp/book/1351)

![ref1](https://www.socym.co.jp/wp-content/uploads/2022/120Px_syoei-1.jpg)

## License

- [MIT](https://github.com/Rei-Ashine/rpn-reckoner/blob/main/LICENSE)
