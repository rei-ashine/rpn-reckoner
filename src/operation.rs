/// Returns the function for a binary operator token, or `None` if the token is not one
pub fn binary_operator(token: &str) -> Option<fn(f64, f64) -> f64> {
    let operator: fn(f64, f64) -> f64 = match token {
        "+" => |left, right| left + right,
        "-" => |left, right| left - right,
        "*" => |left, right| left * right,
        "/" => |left, right| left / right,
        "%" => |left, right| left % right,
        "^" | "**" => f64::powf,
        _ => return None,
    };
    Some(operator)
}

/// Returns `n!` if `n` is a non-negative integer, or `None` otherwise.
///
/// Results above `170!` do not fit in an `f64` and are returned as infinity.
pub fn factorial(n: f64) -> Option<f64> {
    // NaN and infinities fail these checks as well
    if !(n >= 0.0 && n.fract() == 0.0) {
        return None;
    }
    if n > 170.0 {
        return Some(f64::INFINITY);
    }

    let n = n as u64;
    Some((1..=n).fold(1.0, |acc, value| acc * value as f64))
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0.0), Some(1.0));
        assert_eq!(factorial(-0.0), Some(1.0));
        assert_eq!(factorial(1.0), Some(1.0));
        assert_eq!(factorial(5.0), Some(120.0));
        assert_eq!(factorial(10.0), Some(3628800.0));
        assert_eq!(factorial(171.0), Some(f64::INFINITY));

        assert_eq!(factorial(2.5), None);
        assert_eq!(factorial(0.4), None);
        assert_eq!(factorial(-1.0), None);
        assert_eq!(factorial(f64::NAN), None);
        assert_eq!(factorial(f64::INFINITY), None);
    }
}
