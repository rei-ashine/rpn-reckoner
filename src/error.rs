use std::fmt;

/// An error returned by [`eval`](crate::eval).
///
/// The [`Display`](fmt::Display) output is a short English message without any prefix,
/// e.g. `Not enough operands for operator '+'`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An operator did not have enough operands on the stack.
    NotEnoughOperands {
        /// The operator token, e.g. `"+"`.
        operator: String,
    },
    /// A token was neither a number nor a known operator.
    InvalidToken(String),
    /// A number token was not finite, e.g. `inf`, `nan` or `1e400`.
    NonFiniteNumber(String),
    /// An operator produced a value that is not finite,
    /// e.g. division by zero, `0 0 /` (NaN) or an overflow.
    NonFiniteResult {
        /// The operator token, e.g. `"/"`.
        operator: String,
    },
    /// No value was left on the stack, e.g. for an empty expression.
    NoResult,
    /// More than one value was left on the stack.
    TooManyValues,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotEnoughOperands { operator } => {
                write!(f, "Not enough operands for operator '{operator}'")
            }
            Error::InvalidToken(token) => write!(f, "Invalid operator: {token}"),
            Error::NonFiniteNumber(token) => write!(f, "Not a finite number: {token}"),
            Error::NonFiniteResult { operator } => {
                write!(f, "Result of operator '{operator}' is not a finite number")
            }
            Error::NoResult => write!(f, "No result."),
            Error::TooManyValues => write!(f, "Too many values in the stack."),
        }
    }
}

impl std::error::Error for Error {}
