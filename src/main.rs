extern crate foldiff;

use foldiff::args::Args;
use std::env;

fn main() -> Result<(), foldiff::Error> {
    let args = env::args();
    let args = Args::new(args)?;

    foldiff::run(&args)?;

    Ok(())
}
