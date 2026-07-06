pub fn factorial(n: f64) -> f64 {
    if n < 0.0 || n.is_nan() {
        return f64::NAN;
    }
    if n > 170.0 {
        return f64::INFINITY;
    }

    let n = n.round() as usize;

    if n == 0 {return 1.0;}

    (1..=n).fold(1.0, |acc, value| acc * value as f64)
}


#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0.0), 1.0);
        assert_eq!(factorial(1.0), 1.0);
        assert_eq!(factorial(5.0), 120.0);
        assert_eq!(factorial(10.0), 3628800.0);
        assert_eq!(factorial(171.0), f64::INFINITY);
        assert_eq!(factorial(f64::INFINITY), f64::INFINITY);
        assert!(factorial(-1.0).is_nan());
        assert!(factorial(f64::NAN).is_nan());
    }
}