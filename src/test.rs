use crate::eval;


#[test]
fn test_binary_operators() {
    assert_eq!(eval("1 2 +"), Ok(3.0));
    assert_eq!(eval("1 2 -"), Ok(-1.0));
    assert_eq!(eval("2 3 *"), Ok(6.0));
    assert_eq!(eval("6 2 /"), Ok(3.0));
    assert_eq!(eval("5 3 %"), Ok(2.0));

    assert_eq!(eval("2 3 ^"), Ok(8.0));
    assert_eq!(eval("5 2 **"), Ok(25.0));
}

#[test]
fn test_factorial_operator() {
    assert_eq!(eval("4 !"), Ok(24.0));
    assert_eq!(eval("4 ! 2 +"), Ok(26.0));
    assert_eq!(eval("6 4 ! +"), Ok(30.0));

    assert_eq!(eval("0 !"), Ok(1.0));
    assert_eq!(eval("inf !"), Ok(f64::INFINITY));
}

#[test]
fn test_compound_expressions() {
    assert_eq!(eval("5 2 - 3 +"), Ok(6.0));
    assert_eq!(eval("5 2 3 ^ +"), Ok(13.0));
}

#[test]
fn test_number_formats() {
    assert_eq!(eval("-3 2 +"), Ok(-1.0));
    assert_eq!(eval("+5"), Ok(5.0));
    assert_eq!(eval("2.5 2 *"), Ok(5.0));
    assert_eq!(eval("1e3 2 *"), Ok(2000.0));
}

#[test]
fn test_white_space() {
    assert_eq!(eval("  1\t2\n+  "), Ok(3.0));
    assert_eq!(eval("\u{3000}1 2 +"), Ok(3.0));
}

#[test]
fn test_errors() {
    assert_eq!(eval("a + 2"), Err("[Error] Invalid operator: a".to_string()));
    assert_eq!(eval(""), Err("[Error] No result.".to_string()));
    assert_eq!(eval("1 2 3"), Err("[Error] Too many values in the stack.".to_string()));

    assert_eq!(eval("+"), Err("[Error] Not enough operands for operator '+'".to_string()));
    assert_eq!(eval("1 +"), Err("[Error] Not enough operands for operator '+'".to_string()));
    assert_eq!(eval("!"), Err("[Error] Not enough operands for operator '!'".to_string()));

    assert_eq!(eval("0 4 !"), Err("[Error] Too many values in the stack.".to_string()));
}
