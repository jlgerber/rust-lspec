#![allow(dead_code,unused_imports, unused_parens)]
#[macro_use]
extern crate nom;

mod parse_string;

mod leveltype;
use leveltype::*;

#[derive(Debug,Eq,PartialEq)]
pub struct LevelSpec {
    show: LevelType,
    sequence: LevelType,
    shot: LevelType,
}

impl LevelSpec {
    pub fn new(in_str: &str) -> LevelSpec {
        // todo. dont unwrap. return a Result<LevelSpec, Box<Error>>
        let results = parse_string::gen_levelspec(in_str).unwrap();
        results.1
    }

    pub fn from_show(show: &str) -> LevelSpec {
        LevelSpec{
            show: if show == "%" { LevelType::Wildcard } else{ LevelType::Term(show.into()) } ,
            sequence: LevelType::None,
            shot: LevelType::None,
        }
    }

    pub fn from_sequence(show: &str, sequence: &str) -> LevelSpec {
        LevelSpec {
            show: if show == "%" { LevelType::Wildcard } else{ LevelType::Term(show.into()) } ,
            sequence: if sequence == "%" { LevelType::Wildcard } else{ LevelType::Term(sequence.into()) },
            shot: LevelType::None,
        }
    }

    pub fn from_shot(show: &str, sequence: &str, shot: &str) -> LevelSpec {
        LevelSpec {
            show: if show == "%" { LevelType::Wildcard } else{ LevelType::Term(show.into()) } ,
            sequence: if sequence == "%" { LevelType::Wildcard } else{ LevelType::Term(sequence.into()) },
            shot: if shot == "%" { LevelType::Wildcard } else{ LevelType::Term(shot.into()) } ,
        }
    }
}

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn from_show() {
        let ls1 = LevelSpec::from_show("DEVIT");
        let expected = LevelSpec {show: LevelType::Term("DEVIT".into()),
                                  sequence: LevelType::None,
                                  shot: LevelType::None};
        assert_eq!(ls1, expected);
    }

     #[test]
    fn from_sequence() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let expected = LevelSpec {show: LevelType::Term("DEVIT".into()),
                                  sequence: LevelType::Term("RD".into()),
                                shot: LevelType::None};
        assert_eq!(ls1, expected);
    }

     #[test]
    fn from_shot() {
        let ls1 = LevelSpec::from_shot("DEVIT", "RD", "9999");
        let expected = LevelSpec {show: LevelType::Term("DEVIT".into()),
                                  sequence: LevelType::Term("RD".into()),
                                shot: LevelType::Term("9999".into())};
        assert_eq!(ls1, expected);
    }

    #[test]
    fn calling_new() {
        let lv = LevelSpec::new("SHOW.%.1000");
        let lv2 = LevelSpec {
                            show: LevelType::Term("SHOW".into()),
                            sequence: LevelType::Wildcard,
                            shot: LevelType::Term("1000".into())
                         };
        assert_eq!(lv, lv2);
    }
}