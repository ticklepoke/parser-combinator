use regex::Regex;

use super::parser_state::ParserState;

// TODO: create constructor functions letters, letter, digits, digit
#[derive(Debug, Clone)]
pub struct BaseMatcherError;

type Result<T> = std::result::Result<T, BaseMatcherError>;

pub fn base_matcher(target: &str) -> Result<impl Fn(ParserState) -> ParserState> {
    let target = target.to_owned();
    let re = Regex::new(target.as_str());
    if re.is_err() {
        return Err(BaseMatcherError);
    }
    let matcher = move |prev_state: ParserState| {
        if prev_state.result.is_none() {
            return prev_state;
        }

        let re = re.clone().unwrap();
        if let Some(match_result) = re.find_at(prev_state.target.as_str(), prev_state.index) {
            return ParserState {
                target: prev_state.target.to_owned(),
                index: prev_state.index + match_result.end() - match_result.start(),
                result: Some(match_result.as_str().to_owned()),
            };
        }

        ParserState {
            target: prev_state.target,
            index: prev_state.index,
            result: None,
        }
    };
    Ok(matcher)
}
