extern crate foldiff;

use std::env;
use foldiff::args::Args;

fn main() -> Result<(), foldiff::Error> {
    println!("Hello, world!");
    let args = env::args();
    let args = Args::new(args)?;
    foldiff::run(args)?;

    Ok(())
}
