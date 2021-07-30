#[derive(Debug, Clone, PartialEq)]
pub struct ParserState {
    pub target: String,
    pub index: usize,
    pub result: Option<String>,
}

impl ParserState {
    pub fn new(target: String, index: usize) -> Self {
        ParserState {
            target,
            index,
            result: Some("".to_owned()),
        }
    }
    pub fn initial_state(target: String) -> Self {
        ParserState::new(target, 0)
    }
}
