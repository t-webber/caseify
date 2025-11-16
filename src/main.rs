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
    reason = "bad lint"
)]

/// Parses the input arguments
mod cli;
/// Parsing logic to perform conversion between cases.
mod parser;
use std::env;

use parser::case::Case;

use crate::cli::Cli;

#[expect(clippy::unwrap_used, reason = "argv always has at least 1 element")]
fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap();
    if let Err(msg) = Cli::parse(&mut args).and_then(|cli| cli.run()) {
        help(&program_name, &msg);
    }
}

/// Returns `true` iff the terminal supports ANSI escope codes.
fn supports_ansi() -> bool {
    env::var("TERM").is_ok_and(|val| !val.is_empty() && val != "dumb")
}

/// Returns the ansi escape codes if they are supported.
fn get_colours() -> (&'static str, &'static str, &'static str, &'static str) {
    if supports_ansi() {
        ("\x1b[31m", "\x1b[32m", "\x1b[35m", "\x1b[0m")
    } else {
        ("", "", "", "")
    }
}

/// Displays the help message to the screen with a given error message.
#[expect(clippy::print_stderr, reason = "goal of the function")]
#[expect(clippy::unwrap_used, reason = "there is more than 1 case")]
fn help(arg0: &str, msg: &str) {
    let (red, green, magenta, nil) = get_colours();
    eprintln!("{red}Failed to run casify: {msg}{nil}\n");
    eprintln!("{magenta}Usage: {arg0} <{green}case{magenta}> [value]{nil}\n");
    let max_len = Case::HELP.iter().map(|(name, _)| name.len()).max().unwrap();
    eprintln!("Possible {green}case{nil} values:");
    for (name, example) in Case::HELP {
        eprintln!("  {green}{name:<max_len$}{nil}  {example}");
    }
}
