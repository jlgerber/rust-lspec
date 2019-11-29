/// A valid input describing a level within the levelspec.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum LevelType {
    Term(String),
    Wildcard,
    None,
}

impl LevelType {
    /// Convert to a string
    pub fn to_string(&self) -> String {
        match *self {
            LevelType::Term(ref val) => val.clone(),
            LevelType::Wildcard => "%".to_string(),
            _ => String::new(),
        }
    }
    /// Convert to a str
    pub fn to_str(&self) -> Option<&str> {
        match *self {
            LevelType::Term(ref val) => Some(val),
            LevelType::Wildcard => Some("%"),
            _ => None,
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

    // /// Convert to a string, stripping the wrapper away. Relatively cheap.
    // pub fn to_str<'a>(&'a self) -> &'a str {
    //     match *self {
    //         LevelType::Term(ref val) => val,
    //         LevelType::Wildcard => "%",
    //         _ => "",
    //     }
    // }
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
