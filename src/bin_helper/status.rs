use std::env;

use crate::parser::case::Case;

/// Maximum length of the case names.
const MAX_CASE_NAME_LEN: usize = {
    let n = Case::HELP.len();
    let mut i = 0;
    let mut max = 0;
    #[expect(clippy::indexing_slicing, reason = "i < n")]
    while i < n {
        let this_len = Case::HELP[i].0.len();
        if this_len > max {
            max = this_len;
        }
        i += 1;
    }
    max
};

/// Status of the command, that determines what to print to the terminal.
#[derive(Debug)]
pub enum Status {
    /// An error occurred.
    Error(String),
    /// Display the help message
    Help,
}

impl Status {
    /// Displays the help message to the screen with a given error message.
    pub fn to_string(&self, arg0: &str) -> String {
        let (red, green, cyan, magenta, nil) = get_colours();
        let usage = format!("{magenta}Usage: {arg0} <{green}case{magenta}> [value] [--help]{nil}");

        match self {
            Self::Error(err) => {
                format!("{red}Failed to run {arg0}: {err}{nil}\n\n{usage}")
            }
            Self::Help => {
                format!(
                    "\
{usage}
{magenta}Omit `value` to read from stdin (e.g. for pipes){nil}

Possible {green}case{nil} values:
{}

Examples
{cyan}$ caseify Camel \"Hello World\"{nil}
# Output: helloWorld
{cyan}$ echo \"hello World\" | caseify Constant{nil}
# Output: HELLO_WORLD",
                    Case::HELP
                        .iter()
                        .map(|(name, example)| {
                            format!("  {green}{name:<MAX_CASE_NAME_LEN$}{nil}  {example}")
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
    }
}

/// Returns `true` iff the terminal supports ANSI escope codes.
fn supports_ansi() -> bool {
    env::var("TERM").is_ok_and(|val| !val.is_empty() && val != "dumb")
}

/// Returns the ansi escape codes if they are supported.
fn get_colours() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if supports_ansi() {
        ("\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[35m", "\x1b[0m")
    } else {
        ("", "", "", "", "")
    }
}
