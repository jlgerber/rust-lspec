use failure::Fail;
use std::{io, num, path::PathBuf };

#[derive(Debug, Fail)]
pub enum LSpecError {
   
    #[fail(display = "Placeholder error")]
    Placeholder,


     #[fail(display = "Boxed Error '{}'", _0)]
    BoxedError(String),

    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    ParseIntError(#[cause] num::ParseIntError),

    #[fail(display = "{}", _0)]
    ParseNomError(#[cause] nom::ErrorKind),

}


impl From<io::Error> for LSpecError {
    fn from(error: io::Error) -> Self {
        LSpecError::IoError(error)
    }
}

impl From<num::ParseIntError> for LSpecError {
    fn from(error: num::ParseIntError) -> Self {
        LSpecError::ParseIntError(error)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for LSpecError {
    fn from(error: std::boxed::Box<dyn std::error::Error> ) -> Self {
        LSpecError::BoxedError(error.to_string())
    }
}

impl std::convert::From<nom::ErrorKind> for LSpecError {
    fn from(error: nom::ErrorKind ) -> Self {
        LSpecError::ParseNomError(error)
    }
}
