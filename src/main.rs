#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(clippy::doc_include_without_cfg, reason = "see issue #13918")]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "I want them all")]
#![allow(
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::mod_module_files,
    reason = "chosen style"
)]

/// Module to handle the binary cli
mod bin_helper;
/// Parsing logic to perform conversion between cases.
mod parser;
use std::env;

use crate::bin_helper::cli::Cli;
use crate::bin_helper::status::Status;

/// Parses the input from the arguments given to the CLI, and runs caseify on it.
#[expect(clippy::unwrap_used, reason = "argv always has at least 1 element")]
fn parse_and_run<Args: Iterator<Item = String>>(mut args: Args) -> Result<(), String> {
    let program_name = args.next().unwrap();
    Cli::parse(&mut args)
        .and_then(|cli| {
            cli.run()
                .map_err(|err| Status::Error(format!("Io Error: {err}")))
        })
        .map_err(|err| err.to_string(&program_name))
}

#[expect(clippy::print_stderr, reason = "cli")]
fn main() {
    if let Err(err) = parse_and_run(env::args()) {
        eprintln!("{err}");
    }
}

#[cfg(test)]
mod test_errors {
    #![expect(clippy::unwrap_used, reason = "tests should fail")]

    use std::env;
    use std::sync::Mutex;

    use crate::parse_and_run;

    static TERM: Mutex<()> = Mutex::new(());

    const HELP: &str = "Usage: caseify <case> [value] [--help]
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
# Output: HELLO_WORLD";

    fn expect_stderr(args: &[&str], output: &str) {
        let x = TERM.lock().unwrap();
        // SAFETY: TERM locked
        unsafe { env::set_var("TERM", "dumb") }

        assert_eq!(
            parse_and_run(args.iter().map(|arg| (*arg).to_owned())),
            Err(output.to_owned())
        );
        drop(x);
    }

    #[test]
    fn help() {
        expect_stderr(&["caseify", "--help"], HELP);
        expect_stderr(&["caseify", "Pascal", "--help"], HELP);
        expect_stderr(&["caseify", "Pascal", "hello, world!", "--help"], HELP);
    }

    fn test_error(args: &[&str], err: &str) {
        expect_stderr(
            args,
            &format!("Failed to run caseify: {err}\n\nUsage: caseify <case> [value] [--help]"),
        );
    }

    #[test]
    fn invalid_case() {
        test_error(&["caseify", "blob"], "blob isn't a valid case");
    }

    #[test]
    fn too_many_args() {
        test_error(
            &["caseify", "Pascal", "hello", "world"],
            "Too many arguments",
        );
    }

    #[test]
    fn missing_case() {
        test_error(&["caseify"], "Missing `case` argument");
    }

    #[test]
    fn program_name() {
        expect_stderr(
            &["some_program_name"],
            "Failed to run some_program_name: Missing `case` argument\n\nUsage: some_program_name <case> [value] [--help]",
        );
    }

    #[test]
    fn ansi() {
        let x = TERM.lock().unwrap();
        // SAFETY: TERM locked
        unsafe { env::set_var("TERM", "alacritty") }

        let args = &["caseify"];

        assert_eq!(
            parse_and_run(args.iter().map(|arg| (*arg).to_owned())),
            Err("\x1b[31mFailed to run caseify: Missing `case` argument\x1b[0m\n\n\x1b[35mUsage: caseify <\x1b[32mcase\x1b[35m> [value] [--help]\x1b[0m".to_owned())
        );
        drop(x);
    }
}
