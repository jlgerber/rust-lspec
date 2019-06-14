//!
//! parse_string parses string and returns levelspec
//!
use nom::*;
use nom::IResult::*;
use std::str;
use crate::{LevelSpec};

named!(alpha_alphanumeric<&str, &str>,
        verify!(alphanumeric, |val: &str| !val.chars().next().unwrap().is_digit(10) )
);

named!(pub gen_levelspec<&str, LevelSpec>,
    alt!(
        complete!(do_parse!(
            show: alt!(alpha_alphanumeric | tag_s!("%")) >>
            tag_s!(".") >>
            seq: alt!(alphanumeric | tag_s!("%")) >>
            tag_s!(".") >>
            shot: alt!(alphanumeric | tag_s!("%")) >> eof!() >>
            (LevelSpec::from_shot(show, seq, shot))
        )) |
        complete!(do_parse!(
            show:  alt!(alpha_alphanumeric | tag_s!("%")) >>
            tag_s!(".") >>
            seq: alt!(alphanumeric | tag_s!("%")) >> eof!() >>
            (LevelSpec::from_sequence(show, seq))
        )) |
        complete!(do_parse!(
            show: alt!(alpha_alphanumeric | tag_s!("%")) >>
             eof!() >>
            (LevelSpec::from_show(show))
        ))
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    fn s(inv: &str) -> String {
        inv.to_string()
    }
    #[test]
    fn test_alphanum() {
        let l = alpha_alphanumeric("a123");
        assert_eq!(l, Done("","a123"));
        let l = alpha_alphanumeric("1a34");
        assert_eq!(l, Error(ErrorKind::Verify));
        let l = alpha_alphanumeric("aaa&");
        assert_eq!(l, Done("&","aaa"));
    }

    #[test]
    fn shot_success() {
        let l = gen_levelspec("MARY.RD.9999");
        let e = LevelSpec::from_shot("MARY", "RD", "9999");
        assert_eq!(l, Done("",e));
    }


    #[test]
    fn show_starts_with_num_failure() {
        let l = gen_levelspec("1MARY");
        assert_eq!(l, Error(ErrorKind::Alt));

        let l = gen_levelspec("1MARY.RD");
        assert_eq!(l, Error(ErrorKind::Alt));

        let l = gen_levelspec("1MARY.RD.9999");
        assert_eq!(l, Error(ErrorKind::Alt));
    }

    #[test]
    fn shot_seq_wildcard_success() {
        let l = gen_levelspec("MARY.%.9999");
        let e = LevelSpec::from_shot("MARY", "%", "9999");
        assert_eq!(l, Done("",e));
    }

    #[test]
    fn shot_seq_wildcard_shot_wc_success() {
        let l = gen_levelspec("MARY.%.%");
        let e = LevelSpec::from_shot("MARY", "%", "%");
        assert_eq!(l, Done("",e));
    }

     #[test]
    fn shot_error() {
        let l = gen_levelspec("MARY.RD.9999@");
        assert_eq!(l, Error(ErrorKind::Alt) );
    }

    #[test]
    fn shot_wildcard_error() {
        let l = gen_levelspec("MARY.RD.%@");
        assert_eq!(l, Error(ErrorKind::Alt) );
    }

    #[test]
    fn seq_success() {
        let l = gen_levelspec("MARY.RD");
        let e = LevelSpec::from_sequence("MARY", "RD");
        assert_eq!(l, Done("",e));
    }


    #[test]
    fn seq_error() {
        let l = gen_levelspec("MARY.RD#");
        assert_eq!(l, Error(ErrorKind::Alt));
    }

    #[test]
    fn seq_error_prefix() {
        let l = gen_levelspec("MARY.#RD#");
        assert_eq!(l, Error(ErrorKind::Alt));
    }

    #[test]
    fn show_success() {
        let l = gen_levelspec("MARY");
        let e = LevelSpec::from_show("MARY");
        assert_eq!(l, Done("",e));
    }
}