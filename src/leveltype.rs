
#[derive(Debug,Clone,Eq,PartialEq)]
pub enum LevelType {
    Term(String),
    Wildcard,
    None
}

impl LevelType {

    pub fn to_string(&self) -> String {
        match *self {
            LevelType::Term(ref val) => val.clone(),
            LevelType::Wildcard => "%".to_string(),
            _ => String::new(),
        }
    }

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
mod tests {
    use super::*;

    #[test]
    fn to_string_term() {
        let l1 = LevelType::Term("Foobar".into());
        let e = l1.to_string();
        assert_eq!("Foobar".to_string(), e);
    }

    #[test]
    fn to_wildcard_term() {
        let l1 = LevelType::Wildcard;
        let e = l1.to_string();
        assert_eq!("%".to_string(), e);
    }

    #[test]
    fn to_none_term() {
        let l1 = LevelType::None;
        let e = l1.to_string();
        assert_eq!("".to_string(), e);
    }

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