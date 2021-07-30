use crate::core::{parser::Parser, parser_state::ParserState};

pub fn sequence(parsers: Vec<Parser>) -> Parser {
    Parser::new(move |prev_state| {
        if prev_state.result.is_none() {
            return prev_state.clone();
        }
        let mut results = Vec::new();
        let mut next_state = prev_state.clone();

        for p in parsers.iter() {
            next_state = (p.parser_fn)(prev_state.clone());
            results.push(next_state.result.unwrap());
        }
        return ParserState {
            result: Some(results.join("")),
            ..next_state
        };
    })
}

pub fn many(parser: Parser) -> Parser {
    Parser::new(move |prev_state| {
        if prev_state.result.is_none() {
            return prev_state.clone();
        }
        
        let mut results = Vec::new();
        let mut next_state = prev_state.clone();
        loop {
           let temp_state = (parser.parser_fn)(next_state.clone());
           
           if temp_state.result.is_some() {
               results.push(temp_state.result.clone().unwrap());
               next_state = temp_state;
           } else {
               break;
           }
        }
        ParserState {
            result: Some(results.join("")),
            ..next_state
        }
    })
}

pub fn alt(parsers: Vec<Parser>) -> Parser {
    Parser::new(move |prev_state| {
        if prev_state.result.is_none() {
            return prev_state.clone();
        }
        
        for p in parsers.iter() {
            let next_state = (p.parser_fn)(prev_state.clone());
            if next_state.result.is_some() {
                return next_state
            }
        }
        panic!("Failed to parse")
    })
}
