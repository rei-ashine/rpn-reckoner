use std::io::{self, BufRead, Write};
use std::process::ExitCode;

const USAGE: &str = "\
Usage: rpn-reckoner [OPTIONS]

Starts an interactive Reverse Polish Notation calculator.
Type an expression such as `3 2 +` and press Enter.
Type q, quit or exit (or press Ctrl-D) to leave.

Options:
  -h, --help     Print this help and exit
  -V, --version  Print the version and exit";

/// What the command-line arguments ask the program to do
#[derive(Debug, PartialEq)]
enum Command {
    Repl,
    Help,
    Version,
    /// An argument that is not an option this program knows
    Unknown(String),
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Command {
    match args.next() {
        None => Command::Repl,
        Some(arg) => match arg.as_str() {
            "-h" | "--help" => Command::Help,
            "-V" | "--version" => Command::Version,
            _ => Command::Unknown(arg),
        },
    }
}

fn main() -> io::Result<ExitCode> {
    match parse_args(std::env::args().skip(1)) {
        Command::Repl => {}
        Command::Help => {
            println!("{USAGE}");
            return Ok(ExitCode::SUCCESS);
        }
        Command::Version => {
            println!("rpn-reckoner {}", env!("CARGO_PKG_VERSION"));
            return Ok(ExitCode::SUCCESS);
        }
        Command::Unknown(arg) => {
            eprintln!("rpn-reckoner: unexpected argument '{arg}'\n\n{USAGE}");
            return Ok(ExitCode::from(2));
        }
    }

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

    repl(&mut io::stdin().lock(), &mut io::stdout())?;
    Ok(ExitCode::SUCCESS)
}

/// Reads expressions line by line from `input` and writes the results to `output`,
/// until a quit command or the end of the input.
fn repl(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<()> {
    let mut line = Vec::new();
    loop {
        write!(output, "\n🚀 RPN > ")?;
        output.flush()?;

        line.clear();
        if input.read_until(b'\n', &mut line)? == 0 {
            // End of input (Ctrl-D): end the prompt line before leaving
            writeln!(output)?;
            break;
        }

        // Report a line that is not valid UTF-8 instead of stopping the REPL
        let Ok(expression) = std::str::from_utf8(&line) else {
            writeln!(output, "\n👻 [Error] Input is not valid UTF-8")?;
            continue;
        };

        if expression.trim().is_empty() {
            continue;
        }
        if is_quit_command(expression) {
            break;
        }

        match rpn_reckoner::eval(expression) {
            Err(e) => writeln!(output, "\n👻 [Error] {e}")?,
            Ok(r) => writeln!(output, "\n✅ {}", format_number(r))?,
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
    if (1e-7..1e16).contains(&value.abs()) {
        value.to_string()
    } else {
        format!("{value:e}")
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, format_number, is_quit_command, parse_args, repl};

    fn args(list: &[&str]) -> impl Iterator<Item = String> {
        list.iter()
            .map(|arg| (*arg).to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn run(input: &[u8]) -> String {
        let mut output = Vec::new();
        repl(&mut &input[..], &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_parse_args() {
        assert_eq!(parse_args(args(&[])), Command::Repl);
        assert_eq!(parse_args(args(&["-h"])), Command::Help);
        assert_eq!(parse_args(args(&["--help"])), Command::Help);
        assert_eq!(parse_args(args(&["-V"])), Command::Version);
        assert_eq!(parse_args(args(&["--version"])), Command::Version);
        assert_eq!(
            parse_args(args(&["1 2 +"])),
            Command::Unknown("1 2 +".to_string())
        );
    }

    #[test]
    fn test_repl() {
        assert_eq!(run(b"1 2 +\nq\n"), "\n🚀 RPN > \n✅ 3\n\n🚀 RPN > ");
        assert_eq!(
            run(b"1 0 /\nexit\n"),
            "\n🚀 RPN > \n👻 [Error] Result of operator '/' is not a finite number\n\n🚀 RPN > "
        );
    }

    #[test]
    fn test_repl_end_of_input() {
        // The prompt line is ended, so a shell prompt does not follow it directly
        assert_eq!(run(b""), "\n🚀 RPN > \n");
        assert_eq!(run(b"1 2 +"), "\n🚀 RPN > \n✅ 3\n\n🚀 RPN > \n");
    }

    #[test]
    fn test_repl_blank_lines() {
        assert_eq!(
            run(b"\n  \t\n1 2 +\nq\n"),
            "\n🚀 RPN > \n🚀 RPN > \n🚀 RPN > \n✅ 3\n\n🚀 RPN > "
        );
    }

    #[test]
    fn test_repl_invalid_utf8() {
        assert_eq!(
            run(b"\xff\n1 2 +\nq\n"),
            "\n🚀 RPN > \n👻 [Error] Input is not valid UTF-8\n\n🚀 RPN > \n✅ 3\n\n🚀 RPN > "
        );
    }

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
