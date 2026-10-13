use crate::{Error, eval};

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
fn test_operator_semantics() {
    // `%` is the remainder of truncated division: the sign follows the dividend
    assert_eq!(eval("-7 3 %"), Ok(-1.0));
    assert_eq!(eval("7 -3 %"), Ok(1.0));
    assert_eq!(eval("7.5 2 %"), Ok(1.5));

    // `^` and `**` are real-number powers
    assert_eq!(eval("2 -2 ^"), Ok(0.25));
    assert_eq!(eval("4 0.5 **"), Ok(2.0));

    // Any finite number syntax accepted by `str::parse::<f64>`
    assert_eq!(eval(".5 .5 +"), Ok(1.0));
    assert_eq!(eval("5. 1 +"), Ok(6.0));
    assert_eq!(eval("1E3"), Ok(1000.0));
}

#[test]
fn test_factorial_operator() {
    assert_eq!(eval("4 !"), Ok(24.0));
    assert_eq!(eval("4 ! 2 +"), Ok(26.0));
    assert_eq!(eval("6 4 ! +"), Ok(30.0));

    assert_eq!(eval("0 !"), Ok(1.0));
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
    assert_eq!(eval("a + 2"), Err(Error::InvalidToken("a".to_string())));
    assert_eq!(eval(""), Err(Error::NoResult));
    assert_eq!(eval("1 2 3"), Err(Error::TooManyValues));

    assert_eq!(eval("+"), Err(not_enough_operands("+")));
    assert_eq!(eval("1 +"), Err(not_enough_operands("+")));
    assert_eq!(eval("!"), Err(not_enough_operands("!")));

    assert_eq!(eval("0 4 !"), Err(Error::TooManyValues));
}

#[test]
fn test_non_finite() {
    // Non-finite numbers are rejected as input
    for token in ["inf", "-inf", "+inf", "infinity", "nan", "NaN", "1e400"] {
        assert_eq!(
            eval(token),
            Err(Error::NonFiniteNumber(token.to_string())),
            "{token}"
        );
    }
    assert_eq!(
        eval("inf !"),
        Err(Error::NonFiniteNumber("inf".to_string()))
    );

    // Non-finite results are errors, wherever they occur
    assert_eq!(eval("1 0 /"), Err(non_finite_result("/")));
    assert_eq!(eval("-1 0 /"), Err(non_finite_result("/")));
    assert_eq!(eval("0 0 /"), Err(non_finite_result("/")));
    assert_eq!(eval("5 0 %"), Err(non_finite_result("%")));
    assert_eq!(eval("-8 0.333 ^"), Err(non_finite_result("^")));
    assert_eq!(eval("1e308 10 *"), Err(non_finite_result("*")));
    assert_eq!(eval("1 0 / -1 ^"), Err(non_finite_result("/")));
    assert_eq!(eval("1 0 / x"), Err(non_finite_result("/")));
    assert_eq!(eval("171 !"), Err(non_finite_result("!")));

    // Large but finite values are still fine
    assert_eq!(eval("1e308"), Ok(1e308));
    assert!(eval("170 !").is_ok_and(f64::is_finite));
}

#[test]
fn test_factorial_operand() {
    for (expression, value) in [
        ("2.5 !", 2.5),
        ("0.4 !", 0.4),
        ("0.5 !", 0.5),
        ("-1 !", -1.0),
    ] {
        assert_eq!(
            eval(expression),
            Err(Error::InvalidFactorialOperand(value)),
            "{expression}"
        );
    }
    assert_eq!(eval("-0 !"), Ok(1.0));
    assert_eq!(eval("1e3 1e3 / !"), Ok(1.0));
}

#[test]
fn test_error_messages() {
    assert_eq!(
        not_enough_operands("+").to_string(),
        "Not enough operands for operator '+'"
    );
    assert_eq!(
        Error::InvalidToken("a".to_string()).to_string(),
        "Invalid operator: a"
    );
    assert_eq!(
        Error::NonFiniteNumber("inf".to_string()).to_string(),
        "Not a finite number: inf"
    );
    assert_eq!(
        non_finite_result("/").to_string(),
        "Result of operator '/' is not a finite number"
    );
    assert_eq!(
        Error::InvalidFactorialOperand(2.5).to_string(),
        "Factorial requires a non-negative integer, got 2.5"
    );
    assert_eq!(Error::NoResult.to_string(), "No result.");
    assert_eq!(
        Error::TooManyValues.to_string(),
        "Too many values in the stack."
    );
}

fn not_enough_operands(operator: &str) -> Error {
    Error::NotEnoughOperands {
        operator: operator.to_string(),
    }
}

fn non_finite_result(operator: &str) -> Error {
    Error::NonFiniteResult {
        operator: operator.to_string(),
    }
}

/// Evaluates many pseudo-random expressions and checks invariants that must hold
/// for every input: `eval` never panics, and every successful result is finite.
#[test]
fn test_random_expressions() {
    const TOKENS: [&str; 24] = [
        "0", "1", "2", "-3", "2.5", "-0", "0.5", "1e3", "1e308", "170", "171", "inf", "nan",
        "1e400", "+", "-", "*", "/", "%", "^", "**", "!", "x", "++",
    ];
    const SEPARATORS: [&str; 4] = [" ", "  ", "\t", "\n"];

    // A fixed-seed linear congruential generator keeps the test deterministic
    let mut state: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = |bound: usize| {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (state >> 33) as usize % bound
    };

    for _ in 0..20_000 {
        let length = next(10);
        let expression = (0..length)
            .map(|_| TOKENS[next(TOKENS.len())])
            .collect::<Vec<_>>()
            .join(SEPARATORS[next(SEPARATORS.len())]);

        match eval(&expression) {
            Ok(value) => assert!(value.is_finite(), "{expression:?} => {value}"),
            Err(error) => assert!(!error.to_string().is_empty(), "{expression:?}"),
        }
    }
}
