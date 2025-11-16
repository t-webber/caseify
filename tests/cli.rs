use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::str;

#[test]
fn stdin() {
    let mut child = Command::new("./target/debug/caseify")
        .arg("--camel")
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
fn no_stdin() {
    let mut child = Command::new("./target/debug/caseify")
        .args(["--camel", "Hello, World!"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
    assert_eq!(output_str, "helloWorld\n");
}
