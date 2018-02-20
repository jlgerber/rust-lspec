extern crate levelspec;
use levelspec::*;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: levelspec <level>");
    } else {
        let ls = LevelSpec::new(&args[1]);
        println!("{:?}", ls);
    }
}
