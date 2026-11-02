use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    println!();
    println!("#########################################");
    println!("#### Reverse Polish Notation Reckoner ###");
    println!("#########################################");
    println!();

    // ASCII art (Font: colossal)
    println!("8888888b.      8888888b.      888b    888");
    println!("888   Y88b     888   Y88b     8888b   888");
    println!("888    888     888    888     88888b  888");
    println!("888   d88P     888   d88P     888Y88b 888");
    println!("8888888P\"      8888888P\"      888 Y88b888");
    println!("888 T88b       888            888  Y88888");
    println!("888  T88b      888            888   Y8888");
    println!("888   T88b     888            888    Y888");
    println!();

    println!("#########################################");
    println!("Type: q, quit or exit<Enter> (or Ctrl-D) to leave.");
    println!();

    loop {
        print!("\n🚀 RPN > ");
        io::stdout().flush()?;
        let mut expression = String::new();

        let bytes_read = io::stdin().read_line(&mut expression)?;
        if bytes_read == 0 {
            break;
        }

        if is_quit_command(&expression) {
            break;
        }

        let solution = rpn_reckoner::eval(&expression);

        match solution {
            Err(e) => println!("\n👻 [Error] {e}"),
            Ok(r) => println!("\n✅ {}", format_number(r)),
        }
    }
    Ok(())
}

/// Returns `true` if the input line asks to leave the REPL.
fn is_quit_command(input: &str) -> bool {
    matches!(input.trim(), "q" | "quit" | "exit")
}

/// Formats a result of [`rpn_reckoner::eval`] for display.
///
/// Negative zero is shown as `0`, and magnitudes that cannot be shown
/// compactly in decimal (>= 1e16 or < 1e-7) use exponent notation.
/// `eval` only returns finite values, so no special case is needed for
/// infinity or NaN.
fn format_number(value: f64) -> String {
    if value == 0.0 {
        // Covers -0.0 as well, e.g. the result of `0 -1 *`
        return "0".to_string();
    }
    if !(1e-7..1e16).contains(&value.abs()) {
        format!("{value:e}")
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{format_number, is_quit_command};

    #[test]
    fn test_is_quit_command() {
        for input in ["q", "quit", "exit", "  q\n", "exit\r\n"] {
            assert!(is_quit_command(input), "{input:?}");
        }
        for input in ["", "Q", "QUIT", "1 2 +", "q 1", "qq"] {
            assert!(!is_quit_command(input), "{input:?}");
        }
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(3.0), "3");
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(-0.0), "0");
        assert_eq!(format_number(1e-7), "0.0000001");
        assert_eq!(format_number(1.5e-7), "0.00000015");
        assert_eq!(format_number(1e-8), "1e-8");
        assert_eq!(format_number(-1e-300), "-1e-300");
        assert_eq!(format_number(9_999_999_999_999_998.0), "9999999999999998");
        assert_eq!(format_number(1e16), "1e16");
        assert_eq!(format_number(-2.5e20), "-2.5e20");
        assert_eq!(format_number(f64::INFINITY), "inf");
        assert_eq!(format_number(f64::NEG_INFINITY), "-inf");
        assert_eq!(format_number(f64::NAN), "NaN");
    }
}
