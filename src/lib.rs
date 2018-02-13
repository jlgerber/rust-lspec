#![allow(dead_code,unused_imports, unused_parens)]
#[macro_use]
extern crate nom;

mod parse_string;

#[derive(Debug,Eq,PartialEq)]
pub enum LevelType {
    Term(String),
    Wildcard,
    None
}

impl LevelType {
    pub fn is_term(&self) -> bool {
        match *self {
            LevelType::Term(_) => true,
            _ => false,
        }
    }

    pub fn is_wildcard(&self) -> bool {
        match *self {
            LevelType::Wildcard => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match *self {
            LevelType::None => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod leveltypetests {
    use super::*;

    #[test]
    fn is_term() {
        let lt = LevelType::Term("foo".into());
        assert!(lt.is_term());
        assert!(!lt.is_wildcard());
        assert!(!lt.is_none());
    }

     #[test]
    fn is_wildcard() {
        let lt = LevelType::Wildcard;
        assert!(lt.is_wildcard());
        assert!(!lt.is_none());
        assert!(!lt.is_term());
    }

     #[test]
    fn is_none() {
        let lt = LevelType::None;
        assert!(!lt.is_wildcard());
        assert!(lt.is_none());
        assert!(!lt.is_term());
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct LevelSpec {
    show: LevelType,
    sequence: LevelType,
    shot: LevelType,
}

impl LevelSpec {
    pub fn new(in_str: &str) -> LevelSpec {
        // todo. dont unwrap. return a Result<LevelSpec, Box<Error>>
        let results = parse_string::levelspec(in_str).unwrap();
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