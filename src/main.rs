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
    println!("8888888PF      8888888P^      888 Y88b888");
    println!("888 T88b       888            888  Y88888");
    println!("888  T88b      888            888   Y8888");
    println!("888   T88b     888            888    Y888");
    println!();

    println!("#########################################");
    println!("Type: q<Enter> to exit.");
    println!();

    loop {
        print!("\n🚀 RPN > ");
        io::stdout().flush()?;
        let mut expression = String::new();

        let bytes_read = io::stdin().read_line(&mut expression)?;
        if bytes_read == 0 {
            break;
        }

        if expression.trim() == "q" {
            break;
        }

        let solution = rpn_reckoner::eval(&expression);

        match solution {
            Err(e) => println!("\n👻 {e}"),
            Ok(r) => println!("\n✅ {}", format_number(r)),
        }
    }
    Ok(())
}

/// Formats a result for display.
///
/// Negative zero is shown as `0`, and finite magnitudes that cannot be shown
/// compactly in decimal (>= 1e16 or < 1e-7) use exponent notation.
fn format_number(value: f64) -> String {
    if value == 0.0 {
        // Covers -0.0 as well, e.g. the result of `0 -1 *`
        return "0".to_string();
    }
    let magnitude = value.abs();
    if value.is_finite() && !(1e-7..1e16).contains(&magnitude) {
        format!("{value:e}")
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::format_number;

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
