use super::parser_state::ParserState;

pub struct Parser {
    pub parser_fn: Box<dyn Fn(ParserState) -> ParserState>,
}

// TODO: create a parser struct with map() map_err()
impl Parser {
    pub fn new(parser_fn: impl Fn(ParserState) -> ParserState + 'static) -> Self {
        Parser {
            parser_fn: Box::new(parser_fn),
        }
    }

    pub fn run(&self, target: &str) -> ParserState {
        let initial_state = ParserState::initial_state(target.to_owned());
        (self.parser_fn)(initial_state)
    }

    pub fn map(&'static self, map_fn: impl Fn(ParserState) -> ParserState + 'static) -> Self {
        let fn_body = move |prev_state| {
            let next_state = (self.parser_fn)(prev_state);
            if next_state.result.is_none() {
                return next_state;
            }
            return map_fn(next_state);
        };
        Parser::new(fn_body)
    }

    pub fn map_err(
        &'static self,
        map_err_fn: impl Fn(ParserState) -> ParserState + 'static,
    ) -> Self {
        let fn_body = move |prev_state| {
            let next_state = (self.parser_fn)(prev_state);
            if next_state.result.is_some() {
                return next_state;
            }
            return map_err_fn(next_state);
        };
        Parser::new(fn_body)
    }
}
