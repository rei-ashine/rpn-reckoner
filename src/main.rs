use std::io::{ self, Write };


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
        if bytes_read == 0 { break; }

        if expression.trim() == "q" { break; }

        let solution = rpn_reckoner::eval(&expression);

        match solution {
            Err(e) => println!("\n👻 {e}"),
            Ok(r) => println!("\n✅ {}", format_number(r)),
        }
    }
    Ok(())
}


/// Formats a result, using exponent notation for magnitudes too large to show exactly
fn format_number(value: f64) -> String {
    if value.is_finite() && value.abs() >= 1e16 {
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
        assert_eq!(format_number(-0.0), "-0");
        assert_eq!(format_number(1e-7), "0.0000001");
        assert_eq!(format_number(9_999_999_999_999_998.0), "9999999999999998");
        assert_eq!(format_number(1e16), "1e16");
        assert_eq!(format_number(-2.5e20), "-2.5e20");
        assert_eq!(format_number(f64::INFINITY), "inf");
        assert_eq!(format_number(f64::NAN), "NaN");
    }
}