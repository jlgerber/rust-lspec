use crate::{parse_string, LSpecError, LevelType};

/// LevelSpec models a shorthand describing one or more paths
/// on disk, characterized by show, sequence, and shot. This abstraction
/// can be thought of as mapping to something like:
/// `${ROOT}/show/sequence/shot`
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct LevelSpec {
    show: LevelType,
    sequence: LevelType,
    shot: LevelType,
}

impl LevelSpec {
    /// New up a levelspec
    pub fn new<I>(in_str: I) -> Result<LevelSpec, LSpecError>
    where
        I: AsRef<str>,
    {
        let results = parse_string::gen_levelspec(in_str.as_ref()).to_result()?;
        Ok(results)
    }

    /// Convert the levelspec to uppercase
    pub fn upper(&mut self) {
        if let LevelType::Term(ref mut show) = self.show {
            *show = show.to_uppercase()
        }
        if let LevelType::Term(ref mut sequence) = self.sequence {
            *sequence = sequence.to_uppercase()
        }
        if let LevelType::Term(ref mut shot) = self.shot {
            *shot = shot.to_uppercase()
        }
    }

    /// Generate a LevelSpec from a provided show
    pub fn from_show<I>(show: I) -> LevelSpec
    where
        I: AsRef<str>,
    {
        let show = show.as_ref();
        LevelSpec {
            show: if show == "%" {
                LevelType::Wildcard
            } else {
                LevelType::Term(show.into())
            },
            sequence: LevelType::None,
            shot: LevelType::None,
        }
    }

    /// New up a LevelSpec from a show and sequence
    pub fn from_sequence<I>(show: I, sequence: I) -> LevelSpec
    where
        I: AsRef<str>,
    {
        let show = show.as_ref();
        let sequence = sequence.as_ref();
        LevelSpec {
            show: if show == "%" {
                LevelType::Wildcard
            } else {
                LevelType::Term(show.into())
            },
            sequence: if sequence == "%" {
                LevelType::Wildcard
            } else {
                LevelType::Term(sequence.into())
            },
            shot: LevelType::None,
        }
    }

    /// New up a shot from a show, sequence, and shot.
    pub fn from_shot<I>(show: I, sequence: I, shot: I) -> LevelSpec
    where
        I: AsRef<str>,
    {
        let show = show.as_ref();
        let sequence = sequence.as_ref();
        let shot = shot.as_ref();
        LevelSpec {
            show: if show == "%" {
                LevelType::Wildcard
            } else {
                LevelType::Term(show.into())
            },
            sequence: if sequence == "%" {
                LevelType::Wildcard
            } else {
                LevelType::Term(sequence.into())
            },
            shot: if shot == "%" {
                LevelType::Wildcard
            } else {
                LevelType::Term(shot.into())
            },
        }
    }

    /// Retrieve the show if it exists. Otherwise return None
    pub fn show(&self) -> Option<&str> {
        self.show.to_str()
    }

    /// Retrieve the sequence as a string wrapped in an Option
    pub fn sequence(&self) -> Option<&str> {
        self.sequence.to_str()
    }

    /// Retrieve the sequence as a string wrapped in an Option
    pub fn shot(&self) -> Option<&str> {
        self.shot.to_str()
    }

    /// return a vector of Strings representing a level
    pub fn to_vec(&self) -> Vec<String> {
        let mut ret = Vec::new();
        let show = self.show.to_string();
        ret.push(show);
        if !self.sequence.is_none() {
            ret.push(self.sequence.to_string());
            if !self.shot.is_none() {
                ret.push(self.shot.to_string());
            }
        }
        ret
    }

    /// Convert to a vector of &str
    pub fn to_vec_str<'a>(&'a self) -> Vec<&'a str> {
        let mut vec_strs = Vec::<&'a str>::new();
        let val = self.show.to_str();
        if val.is_some() {
            vec_strs.push(val.unwrap());
            let val = self.sequence.to_str();
            if val.is_some() {
                vec_strs.push(val.unwrap());
                let val = self.shot.to_str();
                if val.is_some() {
                    vec_strs.push(val.unwrap());
                }
            }
        }
        vec_strs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_from_show_str() {
        let ls1 = LevelSpec::from_show("DEVIT");
        let expected = LevelSpec {
            show: LevelType::Term("DEVIT".into()),
            sequence: LevelType::None,
            shot: LevelType::None,
        };
        assert_eq!(ls1, expected);
    }

    #[test]
    fn returns_some_str_when_show_method_called() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let show = ls1.show();
        assert_eq!(show, Some("DEVIT"));
    }

    #[test]
    fn returns_some_str_when_seq_method_called() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let seq = ls1.sequence();
        assert_eq!(seq, Some("RD"));
    }

    #[test]
    fn returns_none_when_seq_doesnt_exist() {
        let ls1 = LevelSpec::from_show("DEVIT");
        let seq = ls1.sequence();
        assert_eq!(seq, None);
    }

    #[test]
    fn returns_some_str_when_shot_method_called() {
        let ls1 = LevelSpec::from_shot("DEVIT", "RD", "0001");
        let shot = ls1.shot();
        assert_eq!(shot, Some("0001"));
    }

    #[test]
    fn shot_none() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let shot = ls1.shot();
        assert_eq!(shot, None);
    }

    #[test]
    fn from_sequence() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let expected = LevelSpec {
            show: LevelType::Term("DEVIT".into()),
            sequence: LevelType::Term("RD".into()),
            shot: LevelType::None,
        };
        assert_eq!(ls1, expected);
    }

    #[test]
    fn from_shot() {
        let ls1 = LevelSpec::from_shot("DEVIT", "RD", "9999");
        let expected = LevelSpec {
            show: LevelType::Term("DEVIT".into()),
            sequence: LevelType::Term("RD".into()),
            shot: LevelType::Term("9999".into()),
        };
        assert_eq!(ls1, expected);
    }

    #[test]
    fn from_shot_upper() {
        let mut ls1 = LevelSpec::from_shot("devit", "rd", "9999");
        ls1.upper();
        let expected = LevelSpec {
            show: LevelType::Term("DEVIT".into()),
            sequence: LevelType::Term("RD".into()),
            shot: LevelType::Term("9999".into()),
        };
        assert_eq!(ls1, expected);
    }

    #[test]
    fn calling_new() {
        let lv = LevelSpec::new("SHOW.%.1000").unwrap();
        let lv2 = LevelSpec {
            show: LevelType::Term("SHOW".into()),
            sequence: LevelType::Wildcard,
            shot: LevelType::Term("1000".into()),
        };
        assert_eq!(lv, lv2);
    }

    #[test]
    fn calling_new_err() {
        let lv = LevelSpec::new("SHOW.%.1000.");

        assert!(lv.is_err());
        let es = format!("{}", lv.unwrap_err());
        assert_eq!(es, "Alternative");
    }

    #[test]
    fn to_vec() {
        let lv = LevelSpec::new("FOO.RD.1000").unwrap();
        let res = lv.to_vec();
        let joined = format!("{}.{}.{}", res[0], res[1], res[2]);
        assert_eq!(joined, "FOO.RD.1000".to_string());
    }

    #[test]
    fn to_vec_str() {
        let lv = LevelSpec::new("FOO.RD.1000").unwrap();
        let res = lv.to_vec_str();
        let expected = vec!["FOO", "RD", "1000"];
        assert_eq!(expected, res);
    }
}
