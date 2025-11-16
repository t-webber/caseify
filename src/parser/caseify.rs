use super::char_type::CharType;

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
#[expect(clippy::unreachable, reason = "logically unreachable")]
fn handle_func<F: Fn(&mut String, char), G: Fn(&mut String, char)>(
    output: &mut String,
    old: &CharType,
    on_word_begin: F,
    on_normal: G,
    is_first_capitalised: bool,
    prev: char,
    next_is_lowercase: bool,
) -> CharType {
    let new = CharType::from(prev);
    match (&old, &new) {
        (CharType::None, _) => {
            if is_first_capitalised {
                push_upper(output, prev);
            } else {
                push_lower(output, prev);
            }
        }
        (_, CharType::None) => unreachable!(),
        (_, CharType::Symbol) => (),
        (CharType::Symbol, _) | (CharType::Lowercase, CharType::CapitalOrNumber) => {
            on_word_begin(output, prev);
        }
        (CharType::CapitalOrNumber, CharType::CapitalOrNumber) if next_is_lowercase => {
            on_word_begin(output, prev);
        }
        (CharType::Lowercase, CharType::Lowercase)
        | (CharType::CapitalOrNumber, CharType::CapitalOrNumber | CharType::Lowercase) => {
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
    let mut old = CharType::None;
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
