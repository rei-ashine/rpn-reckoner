# Changelog

## Unreleased

- Documentation: describe the operator rules (`%` keeps the sign of the dividend,
  `^`/`**` are real-number powers) and the accepted number syntax.
- README: show a text transcript of the REPL instead of an outdated screenshot.
- Releases are now published from GitHub Actions using crates.io trusted publishing.

## 2.0.0 (2026-10-12)

### Breaking changes

#### `eval` returns a typed error

`rpn_reckoner::eval` now returns `Result<f64, rpn_reckoner::Error>` instead of `Result<f64, String>`.

`Error` is a `#[non_exhaustive]` enum that implements `Display` and `std::error::Error`:

| Variant | `Display` message |
|---|---|
| `NotEnoughOperands { operator }` | `Not enough operands for operator '+'` |
| `InvalidToken(token)` | `Invalid operator: x` |
| `NonFiniteNumber(token)` (new) | `Not a finite number: inf` |
| `NonFiniteResult { operator }` (new) | `Result of operator '/' is not a finite number` |
| `InvalidFactorialOperand(value)` (new) | `Factorial requires a non-negative integer, got 2.5` |
| `NoResult` | `No result.` |
| `TooManyValues` | `Too many values in the stack.` |

The messages of the existing errors keep their previous wording,
but no longer start with `[Error] `.
The REPL still prints that prefix, so its error lines look the same as before.

#### Only finite numbers are accepted and produced

- Number tokens that are not finite (`inf`, `-inf`, `infinity`, `nan`, `1e400`, ...) are rejected.
- Any operator result that is not finite (division by zero, `0 0 /`, overflow, ...) is an error,
  reported at the operator that produced it. So `eval` only ever returns finite values.

#### Factorial is defined only for non-negative integers

- `!` on a value that is not a non-negative integer is an error.
  Previously such values were silently rounded (`2.5 !` gave `6`), or produced `NaN` for negative values.
- `-0 !` is still `1`, and `170 !` is still the largest factorial that fits in an `f64`.

#### Before and after

| Expression | 1.x | 2.0.0 |
|---|---|---|
| `1 0 /` | `Ok(inf)` | `Err(NonFiniteResult { operator: "/" })` |
| `0 0 /` | `Ok(NaN)` | `Err(NonFiniteResult { operator: "/" })` |
| `5 0 %` | `Ok(NaN)` | `Err(NonFiniteResult { operator: "%" })` |
| `1e308 10 *` | `Ok(inf)` | `Err(NonFiniteResult { operator: "*" })` |
| `171 !` | `Ok(inf)` | `Err(NonFiniteResult { operator: "!" })` |
| `1 0 / -1 ^` | `Ok(0.0)` | `Err(NonFiniteResult { operator: "/" })` |
| `inf` | `Ok(inf)` | `Err(NonFiniteNumber("inf"))` |
| `nan` | `Ok(NaN)` | `Err(NonFiniteNumber("nan"))` |
| `1e400` | `Ok(inf)` | `Err(NonFiniteNumber("1e400"))` |
| `2.5 !` | `Ok(6.0)` | `Err(InvalidFactorialOperand(2.5))` |
| `-1 !` | `Ok(NaN)` | `Err(InvalidFactorialOperand(-1.0))` |

Note `1 0 / -1 ^`: in 1.x an intermediate infinity could disappear and give a finite result.
In 2.0.0 the expression fails at the operator that first produced a non-finite value.

### Migration

- Code that only uses `unwrap()`, `expect()` or `is_ok()` keeps compiling.
- `?` keeps compiling when the caller's error type can be built from `rpn_reckoner::Error`
  (for example `Box<dyn std::error::Error>`). If the caller returns `Result<_, String>`,
  convert with `.map_err(|e| e.to_string())`, or change the error type to `rpn_reckoner::Error`.
- Code that names `Result<f64, String>` for the return value of `eval` must use `rpn_reckoner::Error`.
- Code that compared error strings should match on the enum instead.
  Because `Error` is `#[non_exhaustive]`, a `match` on it needs a wildcard arm.
- `error.to_string()` no longer includes `[Error] `; add it yourself if you displayed it.
- Code that relied on `inf`/`NaN` results, or on rounding in `!`, must handle the new errors.

### Other changes

- REPL: very small results (magnitude below `1e-7`) are shown in exponent notation, and `-0` is shown as `0`.
- Added the `mathematics` category to the package metadata.

## 1.0.1 (2026-10-11)

- Declare `rust-version = "1.85"` and include `LICENSE` in the published package.
- REPL: large results (magnitude `1e16` or more) are shown in exponent notation.
- Documentation, README and internal cleanups; no change to `eval` results.
