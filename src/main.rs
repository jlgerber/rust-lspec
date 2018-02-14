extern crate levelspec;
use levelspec::*;

fn main() {
    let ls = LevelSpec::new("mary.rd.9999");
    println!("{:?}", ls);
}
