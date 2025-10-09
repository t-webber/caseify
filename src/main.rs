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
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::missing_inline_in_public_items,
    clippy::question_mark_used,
    reason = "bad lint"
)]

use clap::{CommandFactory as _, Parser};
use std::{
    io::{self, BufRead as _},
    process::exit,
};

use caseify::Caseify as _;

/// Converts a string to a certain case. Choose a case with the options below.
///
/// Either provide a value, or it will read for stdin (e.g. for `echo text | caseify`)
#[derive(Parser, Debug)]
#[expect(clippy::struct_excessive_bools, reason = "CLI")]
struct Args {
    /// `thisIsCamelCase`
    #[arg(short, long)]
    camel: bool,
    /// `This Is Capitalised Case`
    #[arg(short = 'a', long, visible_alias = "cap")]
    capitalised: bool,
    /// `THIS_IS_CONSTANT_CASE` (or `UPPER_CASE`)
    #[arg(short = 'o', long, visible_alias = "const")]
    constant: bool,
    /// `this.is.dot.case`
    #[arg(short, long)]
    dot: bool,
    /// `this-is-kebab-case` (or `dashed-case`)
    #[arg(short, long)]
    kebab: bool,
    /// `ThisIsPascalCase`
    #[arg(short, long)]
    pascal: bool,
    /// `This is sentence case`
    #[arg(short = 'e', long)]
    sentence: bool,
    /// `this_is_snake_case`
    #[arg(short, long)]
    snake: bool,
    /// If no value is provided, reads from stdin (e.g. for pipes).
    value: Option<String>,
}

impl Args {
    /// Applies the selected case to the provided value.
    fn apply_case(&self, value: &str) -> String {
        if self.camel {
            value.to_camel_case()
        } else if self.pascal {
            value.to_pascal_case()
        } else if self.snake {
            value.to_snake_case()
        } else if self.kebab {
            value.to_kebab_case()
        } else if self.sentence {
            value.to_sentence_case()
        } else if self.constant {
            value.to_constant_case()
        } else if self.capitalised {
            value.to_capitalised_case()
        } else if self.dot {
            value.to_dot_case()
        } else {
            panic("No output case provided.")
        }
    }

    /// Runs the command, processing input and applying the selected case.
    ///
    /// This function reads from stdin if no value is provided, or uses the provided value.
    #[expect(clippy::print_stdout, reason = "this is a CLI")]
    fn run(&self) -> Result<(), io::Error> {
        let nb = [
            self.camel,
            self.pascal,
            self.snake,
            self.kebab,
            self.sentence,
            self.constant,
            self.capitalised,
            self.dot,
        ]
        .iter()
        .filter(|x| **x)
        .count();

        if nb >= 2 {
            panic("You must provide 1 and 1 only output case.")
        } else if nb == 0 {
            panic("No output case provided. Please choose to what case you want to convert to.")
        } else {
            if let Some(value) = &self.value {
                println!("{}", self.apply_case(value));
            } else {
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    println!("{}", self.apply_case(&line?));
                }
            }

            Ok(())
        }
    }
}

/// Panics with a formatted error message and prints the help message.
#[expect(unused_must_use, reason = "don't crash in panic")]
#[expect(clippy::print_stderr, reason = "reason of the function")]
#[expect(clippy::exit, reason = "panic with trace isn't user-friendly")]
fn panic(msg: &str) -> ! {
    Args::command().print_help();
    eprintln!("\x1b[31m\n{msg}\x1b[0m");
    exit(1);
}

fn main() {
    if let Err(err) = Args::parse().run() {
        panic(&format!("Failed to read input: pipe broken: {err}.\n"));
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser as _;

    fn test(args: &[&str], input: &str, output: &str) {
        assert_eq!(Args::parse_from(args).apply_case(input), output);
    }

    #[test]
    fn cli_testing() {
        test(
            &["", "--camel", "this_is_snake_case"],
            "this_is_snake_case",
            "thisIsSnakeCase",
        );
        test(
            &["", "--pascal", "this_is_snake_case"],
            "this_is_snake_case",
            "ThisIsSnakeCase",
        );
        test(
            &["", "--snake", "ThisIsCamelCase"],
            "ThisIsCamelCase",
            "this_is_camel_case",
        );
        test(
            &["", "--kebab", "This Is Capitalised Case"],
            "This Is Capitalised Case",
            "this-is-capitalised-case",
        );
        test(
            &["", "--constant", "this is sentence case"],
            "this is sentence case",
            "THIS_IS_SENTENCE_CASE",
        );
        test(
            &["", "--capitalised", "this.is.dot.case"],
            "this.is.dot.case",
            "This Is Dot Case",
        );
        test(
            &["", "--sentence", "THIS_IS_CONSTANT_CASE"],
            "THIS_IS_CONSTANT_CASE",
            "This is constant case",
        );
        test(
            &["", "--dot", "This Is Kebab Case"],
            "This Is Capitalised Case",
            "this.is.capitalised.case",
        );
    }
}
