#![allow(dead_code, reason = "in bin but not in lib")]

use super::caseify::Caseify as _;

/// Creates the [`Case`] struct and its methods.
macro_rules! make_case {
    ($($upper:ident: $lower:ident: $doc:literal),*) => {
        /// Supported cases
        #[derive(Copy, Clone, Debug)]
        #[non_exhaustive]
        pub enum Case {
            $(
                #[doc = $doc]
                $upper
            ),*
        }

        impl Case {
            /// List of supports cases
            pub(crate) const HELP: &[(&str, &str)] = &[$((stringify!($upper), $doc)),*];


            /// Transforms the input in the given case and returns it.
            #[must_use]
            pub fn caseify(self, input: &str) -> String {
                match self {
                    $(Self::$upper => input.$lower(),)*
                }
            }

            /// Creates a [`Case`] type by parsing from a string.
            pub(crate) fn maybe_from(case: &str) -> Option<Self> {
                match case {
                    $(stringify!($upper) => Some(Self::$upper),)*
                    _ => None
                }
            }
        }

    };
}

make_case!(
    Camel: to_camel_case: "`camelCase`",
    Capitalised: to_capitalised_case: "`Capitalised Case`",
    Constant: to_constant_case: "`CONSTANT_CASE`",
    Dot: to_dot_case: "`dot.case`",
    Kebab: to_kebab_case: "`kebab-case`",
    Pascal: to_pascal_case: "`PascalCase`",
    Sentence: to_sentence_case: "`Sentence case`",
    Snake: to_snake_case: "`snake_case`"
);
