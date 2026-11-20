use std::io::{self, BufRead, IsTerminal, Write};
use std::process::ExitCode;

const USAGE: &str = "\
Usage: rpn-reckoner [OPTIONS] [EXPRESSION...]

With no EXPRESSION, starts an interactive Reverse Polish Notation calculator.
Type an expression such as `3 2 +` and press Enter.
Type q, quit or exit (or press Ctrl-D) to leave.

With an EXPRESSION, evaluates it, prints the result and exits.
Quote it so that the shell does not expand `*`:
  rpn-reckoner '3 4 *'

When the input is not a terminal (e.g. a pipe), evaluates each line
and prints one result per line, without the banner and prompt.

Options:
  -h, --help     Print this help and exit
  -V, --version  Print the version and exit

Exit status: 0 on success, 1 if an expression fails, 2 for an unknown option.";

// ASCII art in the colossal figlet font
const BANNER: &str = r#"
#########################################
#### Reverse Polish Notation Reckoner ###
#########################################

8888888b.      8888888b.      888b    888
888   Y88b     888   Y88b     8888b   888
888    888     888    888     88888b  888
888   d88P     888   d88P     888Y88b 888
8888888P"      8888888P"      888 Y88b888
888 T88b       888            888  Y88888
888  T88b      888            888   Y8888
888   T88b     888            888    Y888

#########################################
Type: q, quit or exit<Enter> (or Ctrl-D) to leave.
"#;

/// What the command-line arguments ask the program to do
#[derive(Debug, PartialEq)]
enum Command {
    /// No arguments: read expressions from the standard input
    Lines,
    Help,
    Version,
    /// Evaluate the arguments, joined with spaces, as one expression
    Eval(String),
    /// An option this program does not know
    Unknown(String),
}

fn parse_args(args: &[String]) -> Command {
    if let [arg] = args {
        match arg.as_str() {
            "-h" | "--help" => return Command::Help,
            "-V" | "--version" => return Command::Version,
            _ => {}
        }
    }
    // Single-dash arguments such as `-3` or `-` are numbers and operators,
    // so only these are treated as options
    let is_option = |arg: &&String| arg.starts_with("--") || matches!(arg.as_str(), "-h" | "-V");
    if let Some(option) = args.iter().find(is_option) {
        return Command::Unknown(option.clone());
    }
    if args.is_empty() {
        Command::Lines
    } else {
        Command::Eval(args.join(" "))
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        // The reader of the output went away (e.g. `| head`): stop quietly
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr(), "rpn-reckoner: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Runs the program; output and input errors are returned instead of panicking
fn run() -> io::Result<ExitCode> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(std::ffi::OsString::into_string)
        .collect();
    let args = match args {
        Ok(args) => args,
        Err(arg) => {
            writeln!(stderr, "rpn-reckoner: argument is not valid UTF-8: {arg:?}")?;
            return Ok(ExitCode::from(2));
        }
    };

    match parse_args(&args) {
        Command::Lines => {}
        Command::Help => {
            writeln!(stdout, "{USAGE}")?;
            return Ok(ExitCode::SUCCESS);
        }
        Command::Version => {
            writeln!(stdout, "rpn-reckoner {}", env!("CARGO_PKG_VERSION"))?;
            return Ok(ExitCode::SUCCESS);
        }
        Command::Eval(expression) => {
            return Ok(match rpn_reckoner::eval(&expression) {
                Ok(value) => {
                    writeln!(stdout, "{}", format_number(value))?;
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    writeln!(stderr, "rpn-reckoner: {error}")?;
                    ExitCode::FAILURE
                }
            });
        }
        Command::Unknown(arg) => {
            writeln!(
                stderr,
                "rpn-reckoner: unexpected argument '{arg}'\n\n{USAGE}"
            )?;
            return Ok(ExitCode::from(2));
        }
    }

    let stdin = io::stdin();
    if !stdin.is_terminal() {
        let ok = eval_lines(&mut stdin.lock(), &mut stdout, &mut stderr)?;
        return Ok(if ok {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        });
    }

    writeln!(stdout, "{BANNER}")?;
    repl(&mut stdin.lock(), &mut stdout)?;
    Ok(ExitCode::SUCCESS)
}

/// Evaluates each line of `input` without a prompt, for use in pipes and scripts.
///
/// Results go to `output`, one per line; errors go to `errors` with their line number.
/// Blank lines are skipped, and a quit command stops reading.
/// Returns `false` if any line failed.
fn eval_lines(
    input: &mut impl BufRead,
    output: &mut impl Write,
    errors: &mut impl Write,
) -> io::Result<bool> {
    let mut ok = true;
    let mut line = Vec::new();
    let mut number = 0;
    loop {
        line.clear();
        if input.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        number += 1;

        let Ok(expression) = std::str::from_utf8(&line) else {
            writeln!(
                errors,
                "rpn-reckoner: line {number}: Input is not valid UTF-8"
            )?;
            ok = false;
            continue;
        };
        if expression.trim().is_empty() {
            continue;
        }
        if is_quit_command(expression) {
            break;
        }

        match rpn_reckoner::eval(expression) {
            Ok(value) => writeln!(output, "{}", format_number(value))?,
            Err(error) => {
                writeln!(errors, "rpn-reckoner: line {number}: {error}")?;
                ok = false;
            }
        }
    }
    Ok(ok)
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
    use super::{Command, eval_lines, format_number, is_quit_command, parse_args, repl};

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|arg| (*arg).to_string()).collect()
    }

    fn run(input: &[u8]) -> String {
        let mut output = Vec::new();
        repl(&mut &input[..], &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    fn run_lines(input: &[u8]) -> (bool, String, String) {
        let (mut output, mut errors) = (Vec::new(), Vec::new());
        let ok = eval_lines(&mut &input[..], &mut output, &mut errors).unwrap();
        (
            ok,
            String::from_utf8(output).unwrap(),
            String::from_utf8(errors).unwrap(),
        )
    }

    #[test]
    fn test_parse_args() {
        assert_eq!(parse_args(&args(&[])), Command::Lines);
        assert_eq!(parse_args(&args(&["-h"])), Command::Help);
        assert_eq!(parse_args(&args(&["--help"])), Command::Help);
        assert_eq!(parse_args(&args(&["-V"])), Command::Version);
        assert_eq!(parse_args(&args(&["--version"])), Command::Version);

        // Arguments are joined into one expression
        assert_eq!(
            parse_args(&args(&["1 2 +"])),
            Command::Eval("1 2 +".to_string())
        );
        assert_eq!(
            parse_args(&args(&["1", "2", "+"])),
            Command::Eval("1 2 +".to_string())
        );
        // Negative numbers and `-` are not options
        assert_eq!(
            parse_args(&args(&["-3", "2", "-"])),
            Command::Eval("-3 2 -".to_string())
        );

        assert_eq!(
            parse_args(&args(&["--foo"])),
            Command::Unknown("--foo".to_string())
        );
        assert_eq!(
            parse_args(&args(&["1", "--help"])),
            Command::Unknown("--help".to_string())
        );
        assert_eq!(
            parse_args(&args(&["-h", "1"])),
            Command::Unknown("-h".to_string())
        );
    }

    #[test]
    fn test_eval_lines() {
        assert_eq!(
            run_lines(b"1 2 +\n\n3 neg\n"),
            (true, "3\n-3\n".to_string(), String::new())
        );
        // A failing line is reported and the others still run
        assert_eq!(
            run_lines(b"1 0 /\n2 2 *\n\xff\n"),
            (
                false,
                "4\n".to_string(),
                "rpn-reckoner: line 1: Result of operator '/' is not a finite number\n\
                 rpn-reckoner: line 3: Input is not valid UTF-8\n"
                    .to_string()
            )
        );
        // A quit command stops reading, and no final newline is needed
        assert_eq!(
            run_lines(b"1 1 +\nq\n5 5 +\n"),
            (true, "2\n".to_string(), String::new())
        );
        assert_eq!(run_lines(b"7"), (true, "7\n".to_string(), String::new()));
        assert_eq!(run_lines(b""), (true, String::new(), String::new()));
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
