use levelspec::{LevelSpec, LSpecError};
use std::env;

fn main() -> Result<(), LSpecError> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: levelspec <level>");
        std::process::exit(1);
    } 
    let mut ls = LevelSpec::new(&args[1])?;
    if let Ok(val) = env::var("LEVELSPEC_UPPER") {
        if val == "1" || val == "true" || val == "yes" {
            ls.upper();
        }
    }

    println!("{:?}", ls);
     
    Ok(())
}
