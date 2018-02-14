#![allow(dead_code,unused_imports, unused_parens)]
#[macro_use]
extern crate nom;

// levelspec parser
// uses nom
mod parse_string;
// LevelType enum defines valid types of
// LevelSpec params
pub mod leveltype;
pub use leveltype::*;
// LevelSpec Struct
pub mod lspec;
pub use lspec::*;
