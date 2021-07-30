# Parser Combinator

A mini parser combinator library written in rust. Inspired by the [Javascript Version](https://github.com/LowLevelJavaScript/Parser-Combinators-From-Scratch).

## Base Parsers

- `string(target: &str)`: Matches on a string `foo`
- `letter()`: Matches on a single letter
- `letters()`: Matches on one or more letters
- `digit()`: Matches on a single digit
- `digits()`: Matches on one or more digits

## Effects

- `many(p: Parser)`: Matches on one or more of `p`'s capture group
- `alt(ps: Vec<Parser>)`: Matches on the first successful capture in `ps`
- `sequence(ps: Vec<Parser>)`: Matches on parsers in `ps` sequentially

## Example

### Basic Usage

```rs
use crate::parsers::letters;
let letter_parser = letters();
assert_eq(letter_parser.run("abc").result, Some("abc".to_owned));
```

### Using effects
```rs
use crate::effects::{alt, sequence};
use crate::parsers::{letters, digits};

// Parse either letters or digits
let letters_or_digits_parser = alt(Vec::from([letters(), digits()]));
assert_eq(letters_or_digits_parser.run("abc").result, Some("abc".to_owned));
assert_eq(letters_or_digits_parser.run("123").result, Some("123".to_owned));

// Parse letters followed by digits
let letters_then_digits_parser = sequence(Vec::from([letters(), digits()]));
assert_eq(letters_then_digits_parser.run("abc123").result, Some("abc123".to_owned));
```