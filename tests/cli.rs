//! End-to-end tests that run the `rpn-reckoner` binary as a separate process.

use std::io::{Read, Write};
use std::process::{Command, Output, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_rpn-reckoner");

/// Runs the binary with `args` and `input` on the standard input (a pipe, not a terminal)
fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).unwrap()
}

fn stderr(output: &Output) -> &str {
    std::str::from_utf8(&output.stderr).unwrap()
}

#[test]
fn test_expression_argument() {
    let output = run(&["3 4 *"], b"");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "12\n");
    assert_eq!(stderr(&output), "");

    // Arguments are joined with spaces, and `-3` is a number, not an option
    let output = run(&["-3", "9", "sqrt", "+"], b"");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "0\n");
}

#[test]
fn test_expression_argument_error() {
    let output = run(&["1 0 /"], b"");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "");
    assert_eq!(
        stderr(&output),
        "rpn-reckoner: Result of operator '/' is not a finite number\n"
    );
}

#[test]
fn test_options() {
    let output = run(&["--version"], b"");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout(&output),
        format!("rpn-reckoner {}\n", env!("CARGO_PKG_VERSION"))
    );

    let output = run(&["-h"], b"");
    assert_eq!(output.status.code(), Some(0));
    assert!(stdout(&output).starts_with("Usage: rpn-reckoner"));

    let output = run(&["--foo"], b"");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(stdout(&output), "");
    assert!(stderr(&output).starts_with("rpn-reckoner: unexpected argument '--foo'\n"));
}

#[test]
fn test_piped_lines() {
    let output = run(&[], b"1 2 +\n\n2 sqrt\nbad\n1e20 1e5 *\n");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout(&output), "3\n1.4142135623730951\n1e25\n");
    assert_eq!(
        stderr(&output),
        "rpn-reckoner: line 4: Invalid operator: bad\n"
    );

    let output = run(&[], b"1 2 +\n");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "3\n");
}

#[cfg(unix)]
#[test]
fn test_argument_not_utf8() {
    use std::os::unix::ffi::OsStrExt;

    let output = Command::new(BIN)
        .arg(std::ffi::OsStr::from_bytes(b"\xff"))
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        stderr(&output),
        "rpn-reckoner: argument is not valid UTF-8: \"\\xFF\"\n"
    );
}

/// Closing the output early, as `| head -1` does, ends the program quietly
#[test]
fn test_output_closed_early() {
    let mut child = Command::new(BIN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Read one line, then close the output while far more than a pipe buffer
    // of results is still to be written
    let mut stdout = child.stdout.take().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        for _ in 0..100_000 {
            if stdin.write_all(b"1 1 +\n").is_err() {
                break;
            }
        }
    });
    let mut first = [0; 2];
    stdout.read_exact(&mut first).unwrap();
    assert_eq!(&first, b"2\n");
    drop(stdout);

    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stderr(&output), "");
}

#[test]
fn test_input_error() {
    // A directory cannot be read as the standard input
    #[cfg(unix)]
    {
        let output = Command::new(BIN)
            .stdin(std::fs::File::open("/").unwrap())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        let message = stderr(&output);
        assert!(message.starts_with("rpn-reckoner: "), "{message}");
        assert!(!message.contains("Os {"), "{message}");
    }
}
