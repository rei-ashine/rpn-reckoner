pub fn factorial(n: f64) -> f64 {
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
    }
}