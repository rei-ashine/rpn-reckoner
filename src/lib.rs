//! # RPN-Reckoner
//! Reverse Polish Notation (: RPN) - Reckoner
//! # Example1
//! ```
//! let solution = rpn_reckoner::eval("1 2 +").unwrap();
//! assert_eq!(solution, 3.0);
//! ```
//! # Example2
//! ```
//! let solution = rpn_reckoner::eval("5 2 - 3 +").unwrap();
//! assert_eq!(solution, 6.0);
//! ```
//! # Example3
//! ```
//! let solution = rpn_reckoner::eval("5 2 3 ^ +").unwrap();
//! assert_eq!(solution, 13.0);
//! ```
//! # Example4
//! ```
//! let solution = rpn_reckoner::eval("4 ! 2 +").unwrap();
//! assert_eq!(solution, 26.0);
//! ```
//! # Errors
//! ```
//! use rpn_reckoner::Error;
//!
//! let error = rpn_reckoner::eval("1 0 /").unwrap_err();
//! assert!(matches!(error, Error::NonFiniteResult { .. }));
//! assert_eq!(
//!     error.to_string(),
//!     "Result of operator '/' is not a finite number"
//! );
//! ```

mod error;
mod operation;

pub use error::Error;

/// Evaluates an RPN expression and returns its value.
///
/// Tokens are separated by white space. Each token is either a finite number
/// (anything `str::parse::<f64>` accepts that is finite, e.g. `-3`, `2.5`, `.5`, `1e3`)
/// or one of these operators:
///
/// | Operator | Meaning |
/// |---|---|
/// | `+`, `-`, `*`, `/` | Arithmetic on `f64` |
/// | `%` | Remainder of truncated division; the sign follows the dividend (`-7 3 %` is `-1`) |
/// | `^`, `**` | Real-number power, as [`f64::powf`] (`2 -2 ^` is `0.25`) |
/// | `!` | Factorial, defined only for non-negative integers |
///
/// Every intermediate result must be finite as well, so the returned value is always finite.
///
/// # Errors
///
/// Returns an [`Error`] when:
/// - an operator does not have enough operands: [`Error::NotEnoughOperands`]
/// - a token is neither a number nor an operator: [`Error::InvalidToken`]
/// - a number is not finite (`inf`, `nan`, `1e400`, ...): [`Error::NonFiniteNumber`]
/// - an operator produces a value that is not finite
///   (division by zero, NaN, overflow): [`Error::NonFiniteResult`]
/// - the operand of `!` is not a non-negative integer: [`Error::InvalidFactorialOperand`]
/// - no value is left on the stack: [`Error::NoResult`]
/// - more than one value is left on the stack: [`Error::TooManyValues`]
pub fn eval(expression: &str) -> Result<f64, Error> {
    let mut stack: Vec<f64> = Vec::new();

    // Separate tokens with white space
    for token in expression.split_whitespace() {
        // Push values onto the stack
        if let Ok(value) = token.parse::<f64>() {
            if !value.is_finite() {
                return Err(Error::NonFiniteNumber(token.to_string()));
            }
            stack.push(value);
            continue;
        }

        // Push calculation results onto the stack
        if token == "!" {
            // Unary Operator (Factorial)
            let Some(value) = stack.pop() else {
                return Err(Error::NotEnoughOperands {
                    operator: token.to_string(),
                });
            };
            let Some(result) = operation::factorial(value) else {
                return Err(Error::InvalidFactorialOperand(value));
            };
            stack.push(finite(result, token)?);
        } else {
            // Binary Operator
            let Some(operator) = operation::binary_operator(token) else {
                return Err(Error::InvalidToken(token.to_string()));
            };
            let (Some(right), Some(left)) = (stack.pop(), stack.pop()) else {
                return Err(Error::NotEnoughOperands {
                    operator: token.to_string(),
                });
            };
            stack.push(finite(operator(left, right), token)?);
        }
    }

    // Exactly one value must remain on the stack
    match stack.as_slice() {
        [] => Err(Error::NoResult),
        [value] => Ok(*value),
        _ => Err(Error::TooManyValues),
    }
}

/// Returns `value` if it is finite, otherwise an error naming the operator that produced it
fn finite(value: f64, operator: &str) -> Result<f64, Error> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::NonFiniteResult {
            operator: operator.to_string(),
        })
    }
}

#[cfg(test)]
mod test;
