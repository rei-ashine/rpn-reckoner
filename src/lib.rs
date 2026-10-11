//! # RPN-Reckoner
//! Reverse Polish Notation (: RPN) - Reckoner
//! # Example1
//! ```
//! let expression = String::from("1 2 +");
//! let solution = rpn_reckoner::eval(&expression).unwrap();
//! println!("{}", solution); // -> 3
//! ```
//! # Example2
//! ```
//! let expression = String::from("5 2 - 3 +");
//! let solution = rpn_reckoner::eval(&expression).unwrap();
//! println!("{}", solution); // -> 6
//! ```
//! # Example3
//! ```
//! let expression = String::from("5 2 3 ^ +");
//! let solution = rpn_reckoner::eval(&expression).unwrap();
//! println!("{}", solution); // -> 13
//! ```
//! # Example4
//! ```
//! let expression = String::from("4 ! 2 +");
//! let solution = rpn_reckoner::eval(&expression).unwrap();
//! println!("{}", solution); // -> 26
//! ```

mod operation;


/// The function computes RPN expressions
pub fn eval(expression: &str) -> Result<f64, String> {
    // Separate tokens with white space
    let tokens = expression.split_whitespace();

    let mut stack: Vec<f64> = Vec::new();
    for token in tokens {
        let t = token.trim();
        if t.is_empty() { continue; }

        // Push values onto the stack
        if let Ok(value) = t.parse::<f64>() {
            stack.push(value);
            continue;
        }

        // Push calculation results onto the stack
        match t {
            // Binary Operator
            "+" | "-" | "*" | "/" | "%" | "^" | "**" => {
                if stack.len() < 2 {
                    return Err(format!("[Error] Not enough operands for operator '{}'", t));
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = match t {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    "%" => left % right,
                    "^" | "**" => left.powf(right),
                    _ => unreachable!(),
                };
                stack.push(result);
            },

            // Unary Operator (Factorial)
            "!" => {
                if stack.is_empty() {
                    return Err(format!("[Error] Not enough operands for operator '{}'", t));
                }
                let value = stack.pop().unwrap();
                stack.push(operation::factorial(value));
            },

            // Error
            _ => return Err(format!("[Error] Invalid operator: {}", t)),
        }
    }

    // Exactly one value must remain on the stack
    match stack.as_slice() {
        [] => Err("[Error] No result.".to_string()),
        [value] => Ok(*value),
        _ => Err("[Error] Too many values in the stack.".to_string()),
    }
}


mod test;