use crate::parsers::{letter, letters, string};

mod core;
mod effects;
mod parsers;

fn main() {
    let dummy_parser = string("abc");
    let letters_parser = letter();
    //println!("{:?}", dummy_parser.run("abcd".to_owned()));
    //println!("{:?}", letters_parser.run("abcd".to_owned()));
}

#[cfg(test)]
mod string_tests {
    #[test]
    fn it_works() {
        //assert_eq!(crate::string("abc")("abc").unwrap(), "abc")
    }
}
