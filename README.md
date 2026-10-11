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

## Development

CI runs the checks below on every pull request.
The Test job runs on stable and on the MSRV (`rust-version` in `Cargo.toml`).
The Lint job (rustfmt, Clippy, rustdoc) uses a pinned Rust toolchain,
set only by `RUST_LINT_TOOLCHAIN` in `.github/workflows/ci.yml`.
The commands read both versions from those files, so they always match CI.

```bash
MSRV=$(sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml)
LINT=$(sed -n 's/.*RUST_LINT_TOOLCHAIN: "\(.*\)".*/\1/p' .github/workflows/ci.yml)
rustup toolchain install "$MSRV" "$LINT" --profile minimal -c clippy,rustfmt

cargo test
cargo +"$MSRV" test
cargo +"$LINT" fmt --check
cargo +"$LINT" clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +"$LINT" doc --no-deps --document-private-items
```

To move to a newer lint toolchain, change `RUST_LINT_TOOLCHAIN`,
run the commands above, and fix any new findings in the same pull request.

The rustfmt commit is listed in `.git-blame-ignore-revs`.
GitHub's blame view skips it automatically; to skip it in local `git blame` too, run:

```bash
git config blame.ignoreRevsFile .git-blame-ignore-revs
```

## Directory Structure

```text
.
├── .git-blame-ignore-revs
├── .github
│   ├── dependabot.yml
│   └── workflows
│       └── ci.yml
├── .gitignore
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
 Markdown                  1          118            0           86           32
 |- BASH                   1           16           15            0            1
 |- Rust                   1            8            8            0            0
 (Total)                              142           23           86           33
─────────────────────────────────────────────────────────────────────────────────
 Rust                      4          249          208            8           33
 |- Markdown               3           41            0           37            4
 (Total)                              290          208           45           37
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                     6          449          247          131           71
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Reference

- [『高効率言語 Rust 書きかた・作りかた』](https://www.socym.co.jp/book/1351)

![ref1](https://www.socym.co.jp/wp-content/uploads/2022/120Px_syoei-1.jpg)

## License

- [MIT](https://github.com/Rei-Ashine/rpn-reckoner/blob/main/LICENSE)
