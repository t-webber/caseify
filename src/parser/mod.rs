#![allow(
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::missing_inline_in_public_items,
    clippy::mod_module_files,
    reason = "chosen style"
)]

/// Defines the [`Case`] struct to list the supported cases.
pub mod case;
/// Defines the [`Caseify`] trait to transform an input to the wanted case.
pub mod caseify;
/// Internal state for parsing and casifying
mod char_type;
