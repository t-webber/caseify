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
    #[expect(clippy::print_stderr, reason = "goal of the function")]
    pub fn eprint(self, arg0: &str) {
        let (red, green, cyan, magenta, nil) = get_colours();
        let usage =
            || eprintln!("{magenta}Usage: {arg0} <{green}case{magenta}> [value] [--help]{nil}");

        match self {
            Self::Error(err) => {
                eprintln!("{red}Failed to run caseify: {err}{nil}\n");
                usage();
            }
            Self::Help => {
                usage();
                eprintln!("\n{magenta}Omit `value` to read from stdin (e.g. for pipes){nil}\n");
                eprintln!("Possible {green}case{nil} values:");
                for (name, example) in Case::HELP {
                    eprintln!("  {green}{name:<MAX_CASE_NAME_LEN$}{nil}  {example}");
                }
                eprintln!("\nExamples");
                eprintln!("{cyan}$ caseify Camel \"Hello World\"{nil}");
                eprintln!("# Output: helloWorld");
                eprintln!("{cyan}$ echo \"hello World\" | caseify Constant{nil}");
                eprintln!("# Output: HELLO_WORLD");
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
