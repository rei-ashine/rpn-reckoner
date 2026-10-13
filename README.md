# [RPN-Reckoner](https://crates.io/crates/rpn-reckoner/)

This package provides a Rust library and  
an executable for doing Reverse Polish Notation (: RPN) calculations.

## Library

This package's library allows for evaluating RPN expressions  
by using the function `rpn_reckoner::eval(expression: &str) -> Result<f64, rpn_reckoner::Error>`.  
This function takes an RPN expression as a string argument and returns a result,  
either the evaluated value as a float or an `rpn_reckoner::Error`.

- Operators: `+`, `-`, `*`, `/`, `%`, `^` (or `**`) and `!` (factorial).
  - `%` is the remainder of truncated division, so its sign follows the dividend (`-7 3 %` is `-1`).
  - `^` and `**` are real-number powers (`2 -2 ^` is `0.25`).
- Numbers are anything Rust's `f64` parser accepts, such as `-3`, `2.5`, `.5` and `1e3`.
- Only finite numbers are accepted and produced:  
  `inf` or `nan` as input, division by zero, and overflow are errors.
- Factorial is defined only for non-negative integers (`2.5 !` is an error).

Version 2.0.0 changed the error type and these rules.
See [CHANGELOG.md](./CHANGELOG.md) for details and migration notes.

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

### Example3 (error handling)

```rust
use rpn_reckoner::Error;

fn main() {
    match rpn_reckoner::eval("1 0 /") {
        Ok(solution) => println!("{}", solution),
        Err(Error::NonFiniteResult { operator }) => {
            println!("'{}' did not produce a finite number", operator); // -> '/' did not produce a finite number
        }
        Err(error) => println!("{}", error),
    }
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

An example session (the start-up banner is omitted):

```text
🚀 RPN > 3 2 +

✅ 5

🚀 RPN > 6 4 ! +

✅ 30

🚀 RPN > 1 0 /

👻 [Error] Result of operator '/' is not a finite number

🚀 RPN > 2.5 !

👻 [Error] Factorial requires a non-negative integer, got 2.5

🚀 RPN > q
```

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
cargo +"$LINT" package --list   # must include LICENSE and CHANGELOG.md
```

To move to a newer lint toolchain, change `RUST_LINT_TOOLCHAIN`,
run the commands above, and fix any new findings in the same pull request.

The rustfmt commit is listed in `.git-blame-ignore-revs`.
GitHub's blame view skips it automatically; to skip it in local `git blame` too, run:

```bash
git config blame.ignoreRevsFile .git-blame-ignore-revs
```

## Releasing

Releases are published to crates.io by `.github/workflows/release.yml`
when a version tag is pushed. It uses crates.io trusted publishing,
so no API token is stored in GitHub or on a local machine.

One-time setup on crates.io (crate settings → Trusted Publishing):
add a GitHub publisher with owner `rei-ashine`, repository `rpn-reckoner`,
workflow `release.yml` and environment `release`.

For each release:

1. Open a pull request that updates `version` in `Cargo.toml`
   and adds a `## <version> (<date>)` entry to `CHANGELOG.md`, and merge it.
2. Tag the merge commit on `main` and push the tag:

   ```bash
   git checkout main && git pull
   git tag v<version>
   git push origin v<version>
   ```

Before publishing, the workflow checks that the tag matches the version in `Cargo.toml`,
that `CHANGELOG.md` has an entry for it, and that the tagged commit is on `main`.
It then runs the tests and `cargo publish --dry-run`,
and only then requests a short-lived token and publishes.

## Directory Structure

```text
.
├── .git-blame-ignore-revs
├── .github
│   ├── dependabot.yml
│   ├── scripts
│   │   └── check-release.sh
│   └── workflows
│       ├── ci.yml
│       └── release.yml
├── .gitignore
├── README.md
├── CHANGELOG.md
├── Cargo.toml
├── LICENSE
└── src
    ├── error.rs
    ├── lib.rs
    ├── main.rs
    ├── operation.rs
    └── test.rs
```

## Reference

- [『高効率言語 Rust 書きかた・作りかた』](https://www.socym.co.jp/book/1351)

![ref1](https://www.socym.co.jp/wp-content/uploads/2022/120Px_syoei-1.jpg)

## License

- [MIT](https://github.com/Rei-Ashine/rpn-reckoner/blob/main/LICENSE)
