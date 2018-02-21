extern crate levelspec;
use levelspec::*;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: levelspec <level>");
    } else {
        // fetch LEVELSPEC_UPPER
        if let Ok(mut ls) = LevelSpec::new(&args[1]) {
            if let Ok(val) = env::var("LEVELSPEC_UPPER") {
                if val == "1" || val == "true" || val == "yes" {
                    ls.upper();
                }
        }
            println!("{:?}", ls);
        } else {
            println!("Error parsing {}", &args[1]);
        }
    }
}
