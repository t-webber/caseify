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
    reason = "bad lint"
)]
#![expect(clippy::unreachable, reason = "I trust")]

/// Type of the read character, which will determine how it is processed
#[derive(Debug)]
enum Type {
    /// The character is an uppercase letter or a number.
    CapitalOrNumber,
    /// The character is a lowercase letter.
    Lowercase,
    /// We are reading before the first character of the string.
    None,
    /// The character is a symbol.
    Symbol,
}

impl From<char> for Type {
    fn from(value: char) -> Self {
        if !value.is_alphanumeric() {
            Self::Symbol
        } else if value.is_lowercase() {
            Self::Lowercase
        } else {
            Self::CapitalOrNumber
        }
    }
}

/// Supported cases
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Case {
    /// `camelCase`
    Camel,
    /// `Capitalised Case`
    Capitalised,
    /// `CONSTANT_CASE`
    Constant,
    /// `dot.case`
    Dot,
    /// `kebab-case`
    Kebab,
    /// `PascalCase`
    Pascal,
    /// `Sentence case`
    Sentence,
    /// `snake_case`
    Snake,
}

impl Case {
    /// Transforms the input in the given case and returns it.
    #[must_use]
    pub fn caseify(self, input: &str) -> String {
        match self {
            Self::Camel => input.to_camel_case(),
            Self::Capitalised => input.to_capitalised_case(),
            Self::Constant => input.to_constant_case(),
            Self::Dot => input.to_dot_case(),
            Self::Kebab => input.to_kebab_case(),
            Self::Pascal => input.to_pascal_case(),
            Self::Sentence => input.to_sentence_case(),
            Self::Snake => input.to_snake_case(),
        }
    }
}

/// Converts the string to various cases.
pub trait Caseify {
    /// Converts the string to `camelCase`.
    fn to_camel_case(&self) -> String;
    /// Converts the string to `Capitalised Case`.
    fn to_capitalised_case(&self) -> String;
    /// Converts the string to `CONSTANT_CASE`.
    fn to_constant_case(&self) -> String;
    /// Converts the string to `dot.case`.
    fn to_dot_case(&self) -> String;
    /// Converts the string to `kebab-case`.
    fn to_kebab_case(&self) -> String;
    /// Converts the string to `PascalCase`.
    fn to_pascal_case(&self) -> String;
    /// Converts the string to `Sentence case`.
    fn to_sentence_case(&self) -> String;
    /// Converts the string to `snake_case`.
    fn to_snake_case(&self) -> String;
}

impl Caseify for str {
    fn to_camel_case(&self) -> String {
        to_new_case(self, false, push_upper, push_lower)
    }

    fn to_capitalised_case(&self) -> String {
        to_new_case(
            self,
            true,
            |output, ch| {
                output.push(' ');
                push_upper(output, ch);
            },
            push_lower,
        )
    }

    fn to_constant_case(&self) -> String {
        to_new_case(
            self,
            true,
            |output, ch| {
                output.push('_');
                push_upper(output, ch);
            },
            push_upper,
        )
    }

    fn to_dot_case(&self) -> String {
        to_new_case(
            self,
            false,
            |output, ch| {
                output.push('.');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_kebab_case(&self) -> String {
        to_new_case(
            self,
            false,
            |output, ch| {
                output.push('-');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_pascal_case(&self) -> String {
        to_new_case(self, true, push_upper, push_lower)
    }

    fn to_sentence_case(&self) -> String {
        to_new_case(
            self,
            true,
            |output, ch| {
                output.push(' ');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_snake_case(&self) -> String {
        to_new_case(
            self,
            false,
            |output, ch| {
                output.push('_');
                push_lower(output, ch);
            },
            push_lower,
        )
    }
}

/// Pushes the lowercase representation of the given character to the output string.
fn push_lower(output: &mut String, ch: char) {
    for byte in ch.to_lowercase() {
        output.push(byte);
    }
}

/// Pushes the uppercase representation of the given character to the output string.
fn push_upper(output: &mut String, ch: char) {
    for byte in ch.to_uppercase() {
        output.push(byte);
    }
}

/// Determines if a character should be interpreted as a word beginning or not, and handles the push.
fn handle_func<F: Fn(&mut String, char), G: Fn(&mut String, char)>(
    output: &mut String,
    old: &Type,
    on_word_begin: F,
    on_normal: G,
    is_first_capitalised: bool,
    prev: char,
    next_is_lowercase: bool,
) -> Type {
    let new = Type::from(prev);
    match (&old, &new) {
        (Type::None, _) => {
            if is_first_capitalised {
                push_upper(output, prev);
            } else {
                push_lower(output, prev);
            }
        }
        (_, Type::None) => unreachable!(),
        (_, Type::Symbol) => (),
        (Type::Symbol, _) | (Type::Lowercase, Type::CapitalOrNumber) => on_word_begin(output, prev),
        (Type::CapitalOrNumber, Type::CapitalOrNumber) if next_is_lowercase => {
            on_word_begin(output, prev);
        }
        (Type::Lowercase, Type::Lowercase)
        | (Type::CapitalOrNumber, Type::CapitalOrNumber | Type::Lowercase) => {
            on_normal(output, prev);
        }
    }
    new
}

/// Converts a string to a new case based on the provided functions for word beginnings and normal characters.
///
/// This is a wrapper function that handles the conversion logic for all the casing methods.
fn to_new_case<F: Fn(&mut String, char), G: Fn(&mut String, char)>(
    value: &str,
    is_first_capitalised: bool,
    on_word_begin: F,
    on_normal: G,
) -> String {
    let mut output = String::with_capacity(value.len());
    let mut old = Type::None;
    let mut current_char: Option<char> = None;
    for ch in value.chars() {
        if let Some(prev) = current_char {
            old = handle_func(
                &mut output,
                &old,
                &on_word_begin,
                &on_normal,
                is_first_capitalised,
                prev,
                ch.is_lowercase(),
            );
        }
        current_char = Some(ch);
    }
    if let Some(prev) = current_char {
        handle_func(
            &mut output,
            &old,
            &on_word_begin,
            &on_normal,
            is_first_capitalised,
            prev,
            false,
        );
    }

    output
}
