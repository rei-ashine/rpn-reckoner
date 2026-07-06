#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval("1 2 +".to_string()), Ok(3.0));
        assert_eq!(eval("1 2 -".to_string()), Ok(-1.0));
        assert_eq!(eval("2 3 *".to_string()), Ok(6.0));
        assert_eq!(eval("6 2 /".to_string()), Ok(3.0));
        assert_eq!(eval("5 3 %".to_string()), Ok(2.0));

        assert_eq!(eval("2 3 ^".to_string()), Ok(8.0));
        assert_eq!(eval("5 2 **".to_string()), Ok(25.0));

        assert_eq!(eval("4 !".to_string()), Ok(24.0));
        assert_eq!(eval("4 ! 2 +".to_string()), Ok(26.0));
        assert_eq!(eval("6 4 ! +".to_string()), Ok(30.0));

        assert_eq!(eval("5 2 - 3 +".to_string()), Ok(6.0));
        assert_eq!(eval("5 2 3 ^ +".to_string()), Ok(13.0));

        assert_eq!(eval("a + 2".to_string()), Err("[Error] Invalid operator: a".to_string()));
        assert_eq!(eval("".to_string()), Err("[Error] No result.".to_string()));
        assert_eq!(eval("1 2 3".to_string()), Err("[Error] Too many values in the stack.".to_string()));

        // Added error edge cases
        assert_eq!(eval("+".to_string()), Err("[Error] Not enough operands for operator '+'".to_string()));
        assert_eq!(eval("1 +".to_string()), Err("[Error] Not enough operands for operator '+'".to_string()));
        assert_eq!(eval("!".to_string()), Err("[Error] Not enough operands for operator '!'".to_string()));

        // Added bugfix test cases
        assert_eq!(eval("0 4 !".to_string()), Err("[Error] Too many values in the stack.".to_string()));
        assert_eq!(eval("0 !".to_string()), Ok(1.0));
    }
}