use std::io::{self, BufRead as _};

use crate::bin_helper::status::Status;
use crate::parser::case::Case;

/// Options passed through the command line
#[derive(Debug)]
pub struct Cli {
    /// Case to convert to
    case: Case,
    /// If no value is provided, reads from stdin (e.g. for pipes).
    value: Option<String>,
}

impl Cli {
    /// Parses the inputs [`Args`] into a [`Cli`] by checking the validity of the [`Args`].
    pub fn parse<Args: Iterator<Item = String>>(mut args: Args) -> Result<Self, Status> {
        let case = Self::parse_case(args.next())?;

        let value = args.next();
        if value.as_ref().is_some_and(|val| val == "--help") {
            return Err(Status::Help);
        }

        if args.next().is_some() {
            return Err(Status::Error("Too many arguments".to_owned()));
        }

        Ok(Self {
            case,
            value: value.as_deref().map(str::to_owned),
        })
    }

    /// Parses the first argument to check if it is a valid case, an option or erroneous.
    fn parse_case(first: Option<String>) -> Result<Case, Status> {
        Err(match first {
            None => Status::Error("Missing `case` argument".to_owned()),
            Some(arg) if arg == "--help" => Status::Help,
            Some(arg) => match Case::maybe_from(&arg) {
                None => Status::Error(format!("{arg} isn't a valid case")),
                Some(case) => return Ok(case),
            },
        })
    }

    /// Runs the command, processing input and applying the selected case.
    ///
    /// This function reads from stdin if no value is provided, or uses the provided value.
    #[expect(clippy::print_stdout, reason = "this is a CLI")]
    pub fn run(&self) -> Result<(), String> {
        if let Some(value) = &self.value {
            println!("{}", self.case.caseify(value));
        } else {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let content = &line.map_err(|err| format!("Io Error: {err}"))?;
                println!("{}", self.case.caseify(content));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;

    #[expect(clippy::unwrap_used, reason = "tests")]
    fn test(args: &[&str], input: &str, output: &str) {
        let owned_args = args.iter().map(|arg| (*arg).to_owned());
        let cli = Cli::parse(owned_args.into_iter()).unwrap();
        assert_eq!(cli.case.caseify(input), output);
    }

    #[test]
    fn camel() {
        test(
            &["Camel", "this_is_snake_case"],
            "this_is_snake_case",
            "thisIsSnakeCase",
        );
    }
    #[test]
    fn pascal() {
        test(
            &["Pascal", "this_is_snake_case"],
            "this_is_snake_case",
            "ThisIsSnakeCase",
        );
    }
    #[test]
    fn snake() {
        test(
            &["Snake", "ThisIsCamelCase"],
            "ThisIsCamelCase",
            "this_is_camel_case",
        );
    }
    #[test]
    fn kebab() {
        test(
            &["Kebab", "This Is Capitalised Case"],
            "This Is Capitalised Case",
            "this-is-capitalised-case",
        );
    }
    #[test]
    fn constant() {
        test(
            &["Constant", "this is sentence case"],
            "this is sentence case",
            "THIS_IS_SENTENCE_CASE",
        );
    }
    #[test]
    fn capitalised() {
        test(
            &["Capitalised", "this.is.dot.case"],
            "this.is.dot.case",
            "This Is Dot Case",
        );
    }
    #[test]
    fn sentence() {
        test(
            &["Sentence", "THIS_IS_CONSTANT_CASE"],
            "THIS_IS_CONSTANT_CASE",
            "This is constant case",
        );
    }
    #[test]
    fn dot() {
        test(
            &["Dot", "This Is Kebab Case"],
            "This Is Capitalised Case",
            "this.is.capitalised.case",
        );
    }
}
