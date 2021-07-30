use crate::core::{base_matcher::base_matcher, parser::Parser, parser_state::ParserState};

pub fn string(target: &str) -> Parser {
    match base_matcher(target) {
        Err(_) => panic!("Invalid pattern"),
        Ok(matcher) => Parser::new(move |prev_state: ParserState| matcher(prev_state)),
    }
}

pub fn letters() -> Parser {
    match base_matcher(r"[a-zA-Z]+") {
        Err(_) => unreachable!(),
        Ok(matcher) => Parser::new(move |prev_state: ParserState| matcher(prev_state)),
    }
}

pub fn letter() -> Parser {
    match base_matcher(r"[a-zA-Z]") {
        Err(_) => unreachable!(),
        Ok(matcher) => Parser::new(move |prev_state: ParserState| matcher(prev_state)),
    }
}

pub fn digits() -> Parser {
    match base_matcher(r"[0-9]+") {
        Err(_) => unreachable!(),
        Ok(matcher) => Parser::new(move |prev_state: ParserState| matcher(prev_state)),
    }
}

pub fn digit() -> Parser {
    match base_matcher(r"[0-9]") {
        Err(_) => unreachable!(),
        Ok(matcher) => Parser::new(move |prev_state: ParserState| matcher(prev_state)),
    }
}

#[cfg(test)]
mod string_tests {
    use crate::{core::parser_state::ParserState, parsers::string};

    #[test]
    fn it_should_match() {
        assert_eq!(
            string("abc").run("abc"),
            ParserState {
                target: "abc".to_owned(),
                index: 3,
                result: Some("abc".to_owned())
            }
        )
    }

    #[test]
    fn it_should_match_partially() {
        assert_eq!(
            string("abc").run("abcdef"),
            ParserState {
                target: "abcdef".to_owned(),
                index: 3,
                result: Some("abc".to_owned())
            }
        )
    }

    #[test]
    fn it_should_not_match() {
        assert_eq!(
            string("abcdef").run("abc"),
            ParserState {
                target: "abc".to_owned(),
                index: 0,
                result: None
            }
        )
    }
}

#[cfg(test)]
mod letters_test {
    use crate::core::parser_state::ParserState;

    #[test]
    fn it_should_match() {
        assert_eq!(
            crate::letters().run("abc"),
            ParserState {
                target: "abc".to_owned(),
                index: 3,
                result: Some("abc".to_owned())
            }
        )
    }

    #[test]
    fn it_should_match_single() {
        assert_eq!(
            crate::letters().run("a"),
            ParserState {
                target: "a".to_owned(),
                index: 1,
                result: Some("a".to_owned())
            }
        )
    }

    #[test]
    fn it_should_match_first() {
        assert_eq!(
            crate::letters().run("a1bc"),
            ParserState {
                target: "a1bc".to_owned(),
                index: 1,
                result: Some("a".to_owned())
            }
        )
    }

    #[test]
    fn it_should_not_match() {
        assert_eq!(
            crate::letters().run("123"),
            ParserState {
                target: "123".to_owned(),
                index: 0,
                result: None
            }
        )
    }
}

#[cfg(test)]
mod letter_test {
    use crate::core::parser_state::ParserState;

    #[test]
    fn it_should_match() {
        assert_eq!(
            crate::letter().run("a"),
            ParserState {
                target: "a".to_owned(),
                index: 1,
                result: Some("a".to_owned())
            }
        )
    }

    #[test]
    fn it_should_match_start() {
        assert_eq!(
            crate::letter().run("abc"),
            ParserState {
                target: "abc".to_owned(),
                index: 1,
                result: Some("a".to_owned())
            }
        )
    }

    #[test]
    fn it_should_not_match() {
        assert_eq!(
            crate::letter().run("123"),
            ParserState {
                target: "123".to_owned(),
                index: 0,
                result: None
            }
        )
    }
}

