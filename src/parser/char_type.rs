/// `CharType` of the read character, which will determine how it is processed
#[derive(Debug)]
pub enum CharType {
    /// The character is an uppercase letter or a number.
    CapitalOrNumber,
    /// The character is a lowercase letter.
    Lowercase,
    /// We are reading before the first character of the string.
    None,
    /// The character is a symbol.
    Symbol,
}

impl From<char> for CharType {
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
