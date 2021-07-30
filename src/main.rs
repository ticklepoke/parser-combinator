use crate::{
    effects::{alt, many},
    parsers::{digit, digits, letter, letters},
};

mod core;
mod effects;
mod parsers;

fn main() {
    let dummy_parser = alt(Vec::from([letters(), digits()]));
    let letters_parser = letter();
    println!("{:?}", dummy_parser.run("123").result);
    println!("{:?}", letters_parser.run("123"));
}

#[cfg(test)]
mod string_tests {
    use crate::{effects::{alt, many, sequence}, parsers::{digit, digits, letter, letters, string}};

    #[test]
    fn it_parses_strings() {
        let string_parser = string("abc");
        assert_eq!(string_parser.run("abc").result, Some("abc".to_owned()));
    }

    #[test]
    fn it_parses_letters() {
        let letter_parser = letter();
        assert_eq!(letter_parser.run("a").result, Some("a".to_owned()));
        assert_eq!(letter_parser.run("abc").result, Some("a".to_owned()));

        let many_letter_parser = many(letter_parser);
        assert_eq!(many_letter_parser.run("a").result, Some("a".to_owned()));
        assert_eq!(many_letter_parser.run("abc").result, Some("abc".to_owned()));

        let letters_parser = letters();
        assert_eq!(letters_parser.run("a").result, Some("a".to_owned()));
        assert_eq!(letters_parser.run("abc").result, Some("abc".to_owned()));
    }

    #[test]
    fn it_parses_digits() {
        let digit_parser = digit();
        assert_eq!(digit_parser.run("1").result, Some("1".to_owned()));
        assert_eq!(digit_parser.run("123").result, Some("1".to_owned()));

        let digits_parser = digits();
        assert_eq!(digits_parser.run("1").result, Some("1".to_owned()));
        assert_eq!(digits_parser.run("123").result, Some("123".to_owned()));

        let many_digit_parser = many(digit());
        assert_eq!(many_digit_parser.run("1").result, Some("1".to_owned()));
        assert_eq!(many_digit_parser.run("123").result, Some("123".to_owned()));
    }

    #[test]
    fn it_returns_none() {
        let letter_parser = letter();
        assert_eq!(letter_parser.run("123").result, None);
        
        let letters_parser = letters();
        assert_eq!(letters_parser.run("123").result, None);

        let digit_parser = digit();
        assert_eq!(digit_parser.run("abc").result, None);

        let digits_parser = digits();
        assert_eq!(digits_parser.run("abc").result, None);
    }
    
    #[test]
    fn it_parses_sequence() {
        let letters_parser = sequence(Vec::from([letters()]));
        assert_eq!(letters_parser.run("a").result, Some("a".to_owned()));
        assert_eq!(letters_parser.run("abc").result, Some("abc".to_owned()));

        let letters_digits = sequence(Vec::from([letters(), digits()]));
        assert_eq!(letters_digits.run("abc123").result, Some("abc123".to_owned()));
        
    }
    
    #[test]
    fn it_parses_alt() {
        let letters_or_digits = alt(Vec::from([letters(), digits()]));
        assert_eq!(letters_or_digits.run("123").result, Some("123".to_owned()));
        assert_eq!(letters_or_digits.run("abc").result, Some("abc".to_owned()));
        assert_eq!(letters_or_digits.run("abc12").result, Some("abc".to_owned()));
    }
}
