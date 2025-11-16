use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, str};

#[test]
fn stdin() {
    let mut child = Command::new("./target/debug/caseify")
        .arg("Camel")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        writeln!(stdin, "Hello, world!").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(output_str, "helloWorld\n");
}

#[test]
fn arg() {
    let child = Command::new("./target/debug/caseify")
        .args(["Camel", "Hello, World!"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(output_str, "helloWorld\n");
}

#[test]
fn help() {
    unsafe { env::set_var("TERM", "dumb") }

    const HELP: &str = "Usage: target/debug/caseify <case> [value] [--help]

Omit `value` to read from stdin (e.g. for pipes)

Possible case values:
  Camel        `camelCase`
  Capitalised  `Capitalised Case`
  Constant     `CONSTANT_CASE`
  Dot          `dot.case`
  Kebab        `kebab-case`
  Pascal       `PascalCase`
  Sentence     `Sentence case`
  Snake        `snake_case`

Examples
$ caseify Camel \"Hello World\"
# Output: helloWorld
$ echo \"hello World\" | caseify Constant
# Output: HELLO_WORLD
";
    let child = Command::new("target/debug/caseify")
        .arg("--help")
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to start process");

    let output_str = str::from_utf8(&child.stderr).expect("Invalid UTF-8 output");
    assert_eq!(output_str, HELP);
}

#[test]
fn error() {
    unsafe { env::set_var("TERM", "dumb") }

    const HELP: &str = "Failed to run caseify: InvalidCase isn't a valid case

Usage: target/debug/caseify <case> [value] [--help]
";

    let child = Command::new("target/debug/caseify")
        .arg("InvalidCase")
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to start process");

    let output_str = str::from_utf8(&child.stderr).expect("Invalid UTF-8 output");
    assert_eq!(output_str, HELP);
}
