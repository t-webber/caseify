# caseify

A fast and simple Rust library and CLI tool for converting strings between different case conventions.

## Features

- **Library**: Trait-based API for easy string case conversion
- **CLI Tool**: Command-line utility for batch processing and pipes
- **Multiple Cases**: Support for 8 different case conventions

## Library Usage

Add it to your `Cargo.toml` or run:

```bash
cargo add caseify
```

Then use it as such:

```rust
use caseify::Caseify;

let input = "lorem Ipsum_dolor.sit-amet";

assert_eq!(input.to_camel_case(), "loremIpsumDolorSitAmet");
assert_eq!(input.to_pascal_case(), "LoremIpsumDolorSitAmet");
assert_eq!(input.to_kebab_case(), "lorem-ipsum-dolor-sit-amet");
assert_eq!(input.to_constant_case(), "LOREM_IPSUM_DOLOR_SIT_AMET");
assert_eq!(input.to_sentence_case(), "Lorem ipsum dolor sit amet");
assert_eq!(input.to_capitalised_case(), "Lorem Ipsum Dolor Sit Amet");
assert_eq!(input.to_dot_case(), "lorem.ipsum.dolor.sit.amet");
```

The library intelligently handles various input formats:

```rust
use caseify::Caseify;

assert_eq!("XMLHttpRequest".to_snake_case(), "xml_http_request");
assert_eq!("linux    _Kernel".to_camel_case(), "linuxKernel");
assert_eq!("some--weird___input".to_pascal_case(), "SomeWeirdInput");
```

You can also use the `Case` enum:

```rust
use caseify::Case;

assert_eq!(Case::Pascal.caseify("hello, world!"), "HelloWorld");
```


## CLI Usage

Install it with:

```bash
cargo install caseify
```

Then use it as such:

```bash
# Convert a single string
caseify --snake "SomeVariableName"
# Output: some_variable_name

# Use with pipes
echo "some text
some_snake_case
AndPascalCase" | caseify --camel
# Output:
# someText
# someSnakeCase
# andPascalCase

# Process multiple lines
cat file.txt | caseify --pascal

# Available options
caseify --help
```
