#[macro_use]
extern crate nom;

mod parse_string;

#[derive(Debug,Eq,PartialEq)]
struct LevelSpec {
    show: String,
    sequence: Option<String>,
    shot: Option<String>,
}

impl LevelSpec {
    pub fn from_show(show: &str) -> LevelSpec {
        LevelSpec{
            show: show.to_string(),
            sequence: None,
            shot: None,
        }
    }

    pub fn from_sequence(show: &str, sequence: &str) -> LevelSpec {
        LevelSpec {
            show: show.into(),
            sequence: Some(sequence.into()),
            shot: None,
        }
    }


    pub fn from_shot(show: &str, sequence: &str, shot: &str) -> LevelSpec {
        LevelSpec {
            show: show.into(),
            sequence: Some(sequence.into()),
            shot: Some(shot.into()),
        }
    }
}

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn from_show() {
        let ls1 = LevelSpec::from_show("DEVIT");
        let expected = LevelSpec {show: "DEVIT".into(), sequence: None, shot: None};
        assert_eq!(ls1, expected);
    }

     #[test]
    fn from_sequence() {
        let ls1 = LevelSpec::from_sequence("DEVIT", "RD");
        let expected = LevelSpec {show: "DEVIT".into(), sequence: Some("RD".into()), shot: None};
        assert_eq!(ls1, expected);
    }

     #[test]
    fn from_shot() {
        let ls1 = LevelSpec::from_shot("DEVIT", "RD", "9999");
        let expected = LevelSpec {show: "DEVIT".into(),
                                  sequence: Some("RD".into()),
                                shot: Some("9999".into())};
        assert_eq!(ls1, expected);
    }
}