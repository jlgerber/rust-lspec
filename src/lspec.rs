use super::*;

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct LevelSpec {
    show: LevelType,
    sequence: LevelType,
    shot: LevelType
}

impl LevelSpec {
    pub fn new(in_str: &str) -> Result<LevelSpec, Box<std::error::Error>> {
        let results = parse_string::gen_levelspec(in_str).to_result()?;
        Ok(results)
    }

    pub fn upper(&mut self) {
        if let LevelType::Term(ref mut show) = self.show {*show = show.to_uppercase()}
        if let LevelType::Term(ref mut sequence) = self.sequence {*sequence = sequence.to_uppercase()}
        if let LevelType::Term(ref mut shot) = self.shot {*shot = shot.to_uppercase()}
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

    pub fn show(&self) -> Option<String> {
        match self.show {
            LevelType::Wildcard => Some(self.show.to_string()),
            LevelType::Term(_) => Some(self.show.to_string()),
            _ => None,
        }
    }

    /// Retrieve the sequence as a string wrapped in an Option
    pub fn sequence(&self) -> Option<String> {
        match self.sequence {
            LevelType::Wildcard => Some(self.sequence.to_string()),
            LevelType::Term(_) => Some(self.sequence.to_string()),
            _ => None,
        }
    }

    /// Retrieve the sequence as a string wrapped in an Option
    pub fn shot(&self) -> Option<String> {
        match self.shot {
            LevelType::Wildcard => Some(self.shot.to_string()),
            LevelType::Term(_) => Some(self.shot.to_string()),
            _ => None,
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

    pub fn to_vec_str<'a>(&'a self) -> Vec<&'a str> {
        let mut v = Vec::<&'a str>::new();
        let val = self.show.to_str();
        if val != "" {
            v.push(val);
            let val = self.sequence.to_str();
            if val != "" {
                v.push(val);
                let val = self.shot.to_str();
                if val != "" {
                    v.push(val);
                }
            }
        }
        v
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
    fn show() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let show = ls1.show();
        assert_eq!(show, Some("DEVIT".to_string()));
    }

   #[test]
    fn seq() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let seq = ls1.sequence();
        assert_eq!(seq, Some("RD".to_string()));
    }

    #[test]
    fn seq_none() {
        let ls1 = LevelSpec::from_show("DEVIT");
        let seq = ls1.sequence();
        assert_eq!(seq, None);
    }


   #[test]
    fn shot() {
        let ls1 = LevelSpec::from_shot("DEVIT", "RD", "0001");
        let shot = ls1.shot();
        assert_eq!(shot, Some("0001".to_string()));
    }

    #[test]
    fn shot_none() {
        let ls1 = LevelSpec::from_sequence("DEVIT","RD");
        let shot = ls1.shot();
        assert_eq!(shot, None);
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
    fn from_shot_upper() {
        let mut ls1 = LevelSpec::from_shot("devit", "rd", "9999");
        ls1.upper();
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


    #[test]
    fn to_vec_str() {
        let lv = LevelSpec::new("FOO.RD.1000").unwrap();
        let res = lv.to_vec_str();
        let expected = vec!["FOO","RD","1000"];
        assert_eq!(expected,res);
    }
}