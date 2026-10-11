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

mod operation;


/// Evaluates an RPN expression and returns its value.
///
/// Tokens are separated by white space. Each token is either a number
/// (anything `str::parse::<f64>` accepts, e.g. `-3`, `2.5`, `1e3`)
/// or one of the operators `+`, `-`, `*`, `/`, `%`, `^` (or `**`) and `!` (factorial).
///
/// # Errors
///
/// Returns an error message when:
/// - an operator does not have enough operands:
///   `"[Error] Not enough operands for operator '<operator>'"`
/// - a token is neither a number nor an operator: `"[Error] Invalid operator: <token>"`
/// - no value is left on the stack: `"[Error] No result."`
/// - more than one value is left on the stack: `"[Error] Too many values in the stack."`
pub fn eval(expression: &str) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    // Separate tokens with white space
    for token in expression.split_whitespace() {
        // Push values onto the stack
        if let Ok(value) = token.parse::<f64>() {
            stack.push(value);
            continue;
        }

        // Push calculation results onto the stack
        if token == "!" {
            // Unary Operator (Factorial)
            let Some(value) = stack.pop() else {
                return Err(format!("[Error] Not enough operands for operator '{token}'"));
            };
            stack.push(operation::factorial(value));
        } else {
            // Binary Operator
            let Some(operator) = operation::binary_operator(token) else {
                return Err(format!("[Error] Invalid operator: {token}"));
            };
            let (Some(right), Some(left)) = (stack.pop(), stack.pop()) else {
                return Err(format!("[Error] Not enough operands for operator '{token}'"));
            };
            stack.push(operator(left, right));
        }
    }

    // Exactly one value must remain on the stack
    match stack.as_slice() {
        [] => Err("[Error] No result.".to_string()),
        [value] => Ok(*value),
        _ => Err("[Error] Too many values in the stack.".to_string()),
    }
}


#[cfg(test)]
mod test;