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

#[expect(clippy::unwrap_used, reason = "argv always has at least 1 element")]
fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap();
    if let Err(status) = Cli::parse(&mut args).and_then(|cli| cli.run().map_err(Status::Error)) {
        status.eprint(&program_name);
    }
}
