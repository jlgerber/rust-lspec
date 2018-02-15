use super::*;

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct LevelSpec {
    show: LevelType,
    sequence: LevelType,
    shot: LevelType,
}

impl LevelSpec {
    pub fn new(in_str: &str) -> Result<LevelSpec, Box<std::error::Error>> {
        let results = parse_string::gen_levelspec(in_str).to_result()?;
        Ok(results)
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
        ret // return ret
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
        let lv = LevelSpec::new("SHOW.%.1000").unwrap();
        let lv2 = LevelSpec {
                            show: LevelType::Term("SHOW".into()),
                            sequence: LevelType::Wildcard,
                            shot: LevelType::Term("1000".into())
                         };
        assert_eq!(lv, lv2);
    }

    #[test]
    fn calling_new_err() {
        let lv = LevelSpec::new("SHOW.%.1000.");

        assert!(lv.is_err());
        let es = format!("{}",lv.unwrap_err());
        assert_eq!(es, "Alternative");
    }

    #[test]
    fn to_vec() {
        let lv = LevelSpec::new("FOO.RD.1000").unwrap();
        let res = lv.to_vec();
        let joined = format!("{}.{}.{}", res[0], res[1], res[2]);
        assert_eq!(joined, "FOO.RD.1000".to_string());
    }
}